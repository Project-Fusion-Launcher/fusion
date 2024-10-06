use reqwest::RequestBuilder;
use serde::Deserialize;
use std::{
    collections::VecDeque,
    path::PathBuf,
    sync::{Arc, Mutex},
};
use tokio::{fs, sync::Notify};

pub struct Download {
    pub request: RequestBuilder,
    pub filename: String,
    pub download_options: DownloadOptions,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DownloadOptions {
    pub install_location: PathBuf,
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

        fs::create_dir_all(&download.download_options.install_location)
            .await
            .unwrap();

        fs::write(
            download
                .download_options
                .install_location
                .join(&download.filename),
            &bytes,
        )
        .await
        .unwrap();

        println!("Downloaded: {}", download.filename);
    }
}

impl Default for DownloadManager {
    fn default() -> Self {
        Self::new()
    }
}
