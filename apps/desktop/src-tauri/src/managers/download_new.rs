use std::{
    collections::VecDeque,
    sync::{Arc, Mutex},
};

use tokio::sync::Notify;
use tokio_util::sync::CancellationToken;

use crate::{common::download::process_download, models::download::Download};

pub struct DownloadManager2 {
    queue: Arc<Mutex<VecDeque<Download>>>,
    queue_notifier: Arc<Notify>,
    cancellation_token: Arc<Mutex<Option<CancellationToken>>>,
}

impl DownloadManager2 {
    pub fn init() -> Self {
        let manager = Self {
            queue: Arc::new(Mutex::new(VecDeque::new())),
            queue_notifier: Arc::new(Notify::new()),
            cancellation_token: Arc::new(Mutex::new(None)),
        };

        manager.process_queue();
        manager
    }

    pub fn enqueue_download(&self, download: Download) {
        println!("Enqueuing download: {:?}", download.game_id);
        let mut queue = self.queue.lock().unwrap();
        queue.push_back(download);
        self.queue_notifier.notify_one();
    }

    pub fn pause_download(&self) {
        if let Some(token) = self.cancellation_token.lock().unwrap().take() {
            token.cancel();
            println!("Download paused");
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

                if let Some(download) = download {
                    let token = CancellationToken::new();
                    {
                        let mut token_lock = cancellation_token.lock().unwrap();
                        *token_lock = Some(token.clone());
                    }

                    let result = process_download(download, token.clone()).await;
                    println!("Download result: {:?}", result);

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
