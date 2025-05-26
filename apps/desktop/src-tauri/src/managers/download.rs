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

                    let progress_agregator = tokio::spawn(async move {
                        while let Some(update) = progress_rx.recv().await {
                            println!(
                                "[Progress Reporter] Downloaded: {}, Written: {}",
                                update.downloaded, update.written
                            );
                        }
                    });

                    let strategy = get_storefront(&download.game_source)
                        .read()
                        .await
                        .download_strategy();

                    let result = strategy
                        .download(&mut download, token.clone(), progress_tx)
                        .await;

                    progress_agregator.await.unwrap();
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
