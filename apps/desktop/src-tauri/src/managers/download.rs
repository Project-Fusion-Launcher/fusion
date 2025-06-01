use crate::{common::result::Result, models::download::Download, storefronts::get_storefront};
use std::{
    collections::VecDeque,
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc, Mutex,
    },
};
use tokio::sync::Notify;
use tokio_util::sync::CancellationToken;

#[tauri::command]
pub async fn pause(
    download_manager: tauri::State<'_, DownloadManager>,
) -> core::result::Result<(), String> {
    /*if download_manager.is_paused() {
        download_manager.resume();
    } else {
        download_manager.pause().await;
    }*/

    Ok(())
}

pub struct DownloadManager {
    downloading: Arc<Mutex<Option<Arc<Download>>>>,
    up_next_queue: Arc<Mutex<VecDeque<Arc<Download>>>>,
    error_queue: Arc<Mutex<VecDeque<Arc<Download>>>>,
    up_next_notifier: Arc<Notify>,

    requeue_notifier: Arc<Notify>,
    cancellation_token: Arc<Mutex<Option<CancellationToken>>>,
    is_paused: Arc<AtomicBool>,
}

impl DownloadManager {
    pub fn init() -> Self {
        let manager = Self {
            downloading: Arc::new(Mutex::new(None)),
            up_next_queue: Arc::new(Mutex::new(VecDeque::new())),
            error_queue: Arc::new(Mutex::new(VecDeque::new())),

            up_next_notifier: Arc::new(Notify::new()),
            requeue_notifier: Arc::new(Notify::new()),
            cancellation_token: Arc::new(Mutex::new(None)),
            is_paused: Arc::new(AtomicBool::new(false)),
        };

        manager.process_queue();
        manager
    }

    pub async fn enqueue(&self, download: Download) -> Result<()> {
        let mut queue = self.up_next_queue.lock().unwrap();
        queue.push_back(Arc::new(download));
        self.up_next_notifier.notify_waiters();
        Ok(())
    }

    pub fn is_paused(&self) -> bool {
        self.is_paused.load(Ordering::SeqCst)
    }

    /*pub async fn pause(&self) {
        self.is_paused.store(true, Ordering::SeqCst);
        let has_active_download = {
            let token_lock = self.cancellation_token.lock().unwrap();
            token_lock.is_some()
        };

        if has_active_download {
            if let Some(token) = self.cancellation_token.lock().unwrap().take() {
                token.cancel();
            }
            self.requeue_notifier.notified().await;
        }
    }

    pub fn resume(&self) {
        self.is_paused.store(false, Ordering::SeqCst);
        self.queue_notifier.notify_one();
    } */

    fn process_queue(&self) {
        let downloading = Arc::clone(&self.downloading);
        let up_next_queue = Arc::clone(&self.up_next_queue);
        let error_queue = Arc::clone(&self.error_queue);
        let up_next_notifier = Arc::clone(&self.up_next_notifier);
        let cancellation_token = Arc::clone(&self.cancellation_token);
        let is_paused = Arc::clone(&self.is_paused);
        // let requeue_notifier = Arc::clone(&self.requeue_notifier);

        tokio::spawn(async move {
            loop {
                let download = {
                    let mut queue_lock = up_next_queue.lock().unwrap();
                    queue_lock.pop_front()
                };

                if let Some(download) = download {
                    let token = CancellationToken::new();

                    {
                        let mut downloading_lock = downloading.lock().unwrap();
                        *downloading_lock = Some(Arc::clone(&download));
                        let mut token_lock = cancellation_token.lock().unwrap();
                        *token_lock = Some(token.clone());
                    }

                    let reporter = tokio::spawn(reporter(Arc::clone(&download)));

                    let strategy = get_storefront(&download.game_source)
                        .read()
                        .await
                        .download_strategy();
                    let result = strategy.start(download, token.clone()).await;

                    reporter.abort();

                    match result {
                        Ok(true) => {
                            println!("Download completed successfully.");
                            let download = {
                                let mut downloading_lock = downloading.lock().unwrap();
                                downloading_lock.take().unwrap()
                            };

                            get_storefront(&download.game_source)
                                .read()
                                .await
                                .post_download(&download.game_id, download.path.clone())
                                .await
                                .unwrap();
                        }
                        Ok(false) => {
                            println!("Download was cancelled or failed.");
                            is_paused.store(true, Ordering::SeqCst);
                        }
                        Err(e) => {
                            println!("Error during download: {:?}", e);
                            let mut downloading_lock = downloading.lock().unwrap();
                            let download = downloading_lock.take().unwrap();
                            let mut error_queue_lock = error_queue.lock().unwrap();
                            error_queue_lock.push_back(download);
                        }
                    }

                    let mut token_lock = cancellation_token.lock().unwrap();
                    *token_lock = None;
                } else {
                    println!("Waiting for downloads...");
                    up_next_notifier.notified().await;
                    println!("Processing downloads...");
                }
            }
        });
    }
}

async fn reporter(download: Arc<Download>) {
    let mut last_downloaded = download.downloaded();
    let mut last_written = download.written();

    loop {
        let downloaded = download.downloaded();
        let written = download.written();

        let downloaded_pct = if download.download_size > 0 {
            downloaded as f64 * 100.0 / download.download_size as f64
        } else {
            0.0
        };

        let written_pct = if download.install_size > 0 {
            written as f64 * 100.0 / download.install_size as f64
        } else {
            0.0
        };

        let delta_download = downloaded.saturating_sub(last_downloaded);
        let delta_write = written.saturating_sub(last_written);

        let download_speed_mbps = (delta_download as f64 * 8.0) / 1_000_000.0;

        println!(
            "[Progress Reporter] Downloaded: {} ({:.2}%), Written: {} ({:.2}%)",
            downloaded, downloaded_pct, written, written_pct
        );

        println!(
            "[Progress Reporter] Network usage: {:.2} Mbs, Disk usage: {:.2} MBs",
            download_speed_mbps,
            delta_write as f64 / 1_000_000.0
        );

        last_downloaded = downloaded;
        last_written = written;

        tokio::time::sleep(std::time::Duration::from_secs(1)).await;
    }
}
