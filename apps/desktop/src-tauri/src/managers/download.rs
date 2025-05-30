use crate::{
    common::result::Result,
    models::download::{Download, DownloadProgress},
    storefronts::get_storefront,
};
use std::{
    collections::VecDeque,
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc, Mutex,
    },
};
use tokio::sync::{mpsc, Notify};
use tokio_util::sync::CancellationToken;

#[tauri::command]
pub async fn pause(
    download_manager: tauri::State<'_, DownloadManager>,
) -> core::result::Result<(), String> {
    if download_manager.is_paused() {
        download_manager.resume();
    } else {
        download_manager.pause();
    }

    Ok(())
}

pub struct DownloadManager {
    queue: Arc<Mutex<VecDeque<Download>>>,
    queue_notifier: Arc<Notify>,
    cancellation_token: Arc<Mutex<Option<CancellationToken>>>,
    is_paused: Arc<AtomicBool>,
}

impl DownloadManager {
    pub fn init() -> Self {
        let manager = Self {
            queue: Arc::new(Mutex::new(VecDeque::new())),
            queue_notifier: Arc::new(Notify::new()),
            cancellation_token: Arc::new(Mutex::new(None)),
            is_paused: Arc::new(AtomicBool::new(false)),
        };

        manager.process_queue();
        manager
    }

    pub async fn enqueue(&self, download: Download) -> Result<()> {
        let mut queue = self.queue.lock().unwrap();
        queue.push_back(download);
        self.resume();
        Ok(())
    }

    pub fn is_paused(&self) -> bool {
        self.is_paused.load(Ordering::SeqCst)
    }

    pub fn pause(&self) {
        if let Some(token) = self.cancellation_token.lock().unwrap().take() {
            token.cancel();
        }
        self.is_paused.store(true, Ordering::SeqCst);
    }

    pub fn resume(&self) {
        self.is_paused.store(false, Ordering::SeqCst);
        self.queue_notifier.notify_one();
    }

    fn process_queue(&self) {
        let queue_clone = Arc::clone(&self.queue);
        let queue_notifier = Arc::clone(&self.queue_notifier);
        let cancellation_token = Arc::clone(&self.cancellation_token);
        let is_paused = Arc::clone(&self.is_paused);

        tokio::spawn(async move {
            loop {
                if is_paused.load(Ordering::SeqCst) {
                    queue_notifier.notified().await;
                }

                let download = {
                    let mut queue_lock = queue_clone.lock().unwrap();
                    queue_lock.pop_front()
                };

                if let Some(mut download) = download {
                    let token = CancellationToken::new();
                    {
                        let mut token_lock = cancellation_token.lock().unwrap();
                        *token_lock = Some(token.clone());
                    }

                    let (progress_tx, mut progress_rx) = mpsc::channel::<DownloadProgress>(50);

                    let download_size = download.download_size;
                    let install_size = download.install_size;
                    let reporter = tokio::spawn(async move {
                        let mut downloaded = 0;
                        let mut written = 0;

                        while let Some(update) = progress_rx.recv().await {
                            let downloaded_pct = if download_size > 0 {
                                update.downloaded as f64 * 100.0 / download_size as f64
                            } else {
                                0.0
                            };

                            let written_pct = if install_size > 0 {
                                update.written as f64 * 100.0 / install_size as f64
                            } else {
                                0.0
                            };

                            let delta_download = update.downloaded.saturating_sub(downloaded);
                            let delta_write = update.written.saturating_sub(written);

                            let download_speed_mbps = (delta_download as f64 * 8.0) / 1_000_000.0;

                            println!(
                                "[Progress Reporter] Downloaded: {} ({:.2}%), Written: {} ({:.2}%)",
                                update.downloaded, downloaded_pct, update.written, written_pct
                            );

                            println!(
                                "[Progress Reporter] Network usage: {:.2} Mbs, Disk usage: {:.2} MBs",
                                download_speed_mbps,
                                delta_write as f64 / 1_000_000.0
                            );

                            downloaded = update.downloaded;
                            written = update.written;
                        }
                    });

                    let strategy = get_storefront(&download.game_source)
                        .read()
                        .await
                        .download_strategy();

                    let result = strategy
                        .start(&mut download, token.clone(), progress_tx)
                        .await;

                    reporter.await.unwrap();
                    println!("Download result: {:?}", result);

                    if !download.completed {
                        let mut queue_lock = queue_clone.lock().unwrap();
                        queue_lock.push_front(download);
                    } else {
                        get_storefront(&download.game_source)
                            .read()
                            .await
                            .post_download(&download.game_id, download.path)
                            .await
                            .unwrap();
                    }

                    let mut token_lock = cancellation_token.lock().unwrap();
                    *token_lock = None;
                } else {
                    println!("Waiting for downloads...");
                    queue_notifier.notified().await;
                    println!("Processing downloads...");
                }
            }
        });
    }
}
