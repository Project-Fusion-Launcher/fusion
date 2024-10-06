use std::{
    collections::VecDeque,
    sync::{Arc, Mutex},
};

use reqwest::RequestBuilder;
use tokio::sync::Notify;

use crate::models::game::Game;

pub struct Download {
    pub request: RequestBuilder,
    pub game: Game,
}

pub struct DownloadManager {
    queue: Arc<Mutex<VecDeque<Download>>>,
    queue_notifier: Arc<Notify>,
}

impl DownloadManager {
    pub fn new() -> Self {
        let manager = Self {
            queue: Arc::new(Mutex::new(VecDeque::new())),
            queue_notifier: Arc::new(Notify::new()),
        };

        manager.process_queue();
        manager
    }

    pub fn enqueue_download(&self, download: Download) {
        self.queue.lock().unwrap().push_back(download);
        self.queue_notifier.notify_one();
    }

    fn process_queue(&self) {
        let queue_clone = self.queue.clone();
        let queue_notifier = self.queue_notifier.clone();

        tokio::spawn(async move {
            loop {
                let download = {
                    let mut queue_lock = queue_clone.lock().unwrap();
                    queue_lock.pop_front()
                };

                if let Some(download) = download {
                    Self::download(download).await;
                } else {
                    queue_notifier.notified().await;
                }
            }
        });
    }

    async fn download(download: Download) {
        let response = download.request.send().await.unwrap();
        let bytes = response.bytes().await.unwrap();

        std::fs::write(
            format!("C:\\Users\\jorge\\Desktop\\{}.zip", download.game.title),
            &bytes,
        )
        .unwrap();
        println!("Downloaded: {}", download.game.title);
    }
}

impl Default for DownloadManager {
    fn default() -> Self {
        Self::new()
    }
}
