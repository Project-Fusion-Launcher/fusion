use crate::{
    common::result::Result,
    models::download::{Download, DownloadProgress},
    storefronts::get_storefront,
};
use std::{
    collections::VecDeque,
    sync::{Arc, Mutex},
};
use tokio::sync::{mpsc, Notify};
use tokio_util::sync::CancellationToken;

pub struct DownloadManager {
    queue: Arc<Mutex<VecDeque<Download>>>,
    queue_notifier: Arc<Notify>,
    cancellation_token: Arc<Mutex<Option<CancellationToken>>>,
}

impl DownloadManager {
    pub fn init() -> Self {
        let manager = Self {
            queue: Arc::new(Mutex::new(VecDeque::new())),
            queue_notifier: Arc::new(Notify::new()),
            cancellation_token: Arc::new(Mutex::new(None)),
        };

        manager.process_queue();
        manager
    }

    pub async fn enqueue_download(&self, download: Download) -> Result<()> {
        let mut queue = self.queue.lock().unwrap();
        queue.push_back(download);
        self.queue_notifier.notify_one();
        Ok(())
    }

    pub fn pause_download(&self) {
        if let Some(token) = self.cancellation_token.lock().unwrap().take() {
            token.cancel();
        }
    }

    fn process_queue(&self) {
        let queue_clone = self.queue.clone();
        let queue_notifier = self.queue_notifier.clone();
        let cancellation_token = self.cancellation_token.clone();

        tokio::spawn(async move {
            loop {
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
                            println!("[Progress Reporter] Downloaded chunk: {}", update.chunk_id);
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

                    {
                        let mut token_lock = cancellation_token.lock().unwrap();
                        *token_lock = None;
                    }
                } else {
                    println!("Waiting for downloads...");
                    queue_notifier.notified().await;
                    println!("Processing downloads...");
                }
            }
        });
    }
}
