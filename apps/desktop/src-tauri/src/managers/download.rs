use reqwest::RequestBuilder;
use serde::Deserialize;
use std::{
    collections::VecDeque,
    path::PathBuf,
    sync::{Arc, Mutex},
};
use tokio::{
    fs::{self, OpenOptions},
    io::AsyncWriteExt,
    sync::{mpsc, Notify},
    task,
};

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
                    println!("Waiting for downloads...");
                    queue_notifier.notified().await;
                    println!("Processing downloads...");
                }
            }
        });
    }

    async fn download(download: Download) {
        let (tx, mut rx) = mpsc::channel(16);

        let downloader = task::spawn(async move {
            let mut response = download.request.send().await.unwrap();

            while let Some(chunk) = response.chunk().await.unwrap() {
                if (tx.send(chunk).await).is_err() {
                    break; // Receiver dropped
                }
            }
        });

        fs::create_dir_all(&download.download_options.install_location)
            .await
            .unwrap();

        let file_path = download
            .download_options
            .install_location
            .join(&download.filename);

        let mut file = OpenOptions::new()
            .create(true)
            .truncate(true)
            .write(true)
            .open(&file_path)
            .await
            .unwrap();

        let writer = task::spawn(async move {
            while let Some(chunk) = rx.recv().await {
                file.write_all(&chunk).await.unwrap();
            }
        });

        downloader.await.unwrap();
        writer.await.unwrap();

        println!("Downloaded: {}", download.filename);
    }
}

impl Default for DownloadManager {
    fn default() -> Self {
        Self::new()
    }
}
