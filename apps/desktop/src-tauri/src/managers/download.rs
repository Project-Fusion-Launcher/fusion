use crate::{
    models::{game::GameSource, payloads::DownloadFinishedPayload},
    storefronts::{itchio, legacygames},
    APP,
};
use reqwest::RequestBuilder;
use serde::Deserialize;
use std::{
    collections::VecDeque,
    path::PathBuf,
    sync::{Arc, Mutex},
};
use tauri::Emitter;
use tokio::{
    fs::{self, OpenOptions},
    io::AsyncWriteExt,
    sync::{mpsc, Notify},
    task,
};

pub struct Download {
    pub request: RequestBuilder,
    pub file_name: String,
    pub game_source: GameSource,
    pub game_id: String,
    pub download_options: DownloadOptions,
    pub md5: Option<String>,
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
    pub fn init() -> Self {
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
                    let path = download.download_options.install_location.clone();
                    let file_name = download.file_name.clone();
                    let game_id = download.game_id.clone();
                    let game_source = download.game_source.clone();

                    Self::download(download).await;

                    let result = match game_source {
                        GameSource::Itchio => {
                            itchio::post_download(&game_id, path, &file_name).await
                        }
                        GameSource::LegacyGames => {
                            legacygames::post_download(&game_id, path, &file_name).await
                        }
                    };

                    if let Err(e) = result {
                        println!("Error post-download: {}", e);
                    }
                    APP.get()
                        .unwrap()
                        .emit(
                            "download-finished",
                            DownloadFinishedPayload {
                                game_id,
                                game_source,
                            },
                        )
                        .unwrap();
                } else {
                    println!("Waiting for downloads...");
                    queue_notifier.notified().await;
                    println!("Processing downloads...");
                }
            }
        });
    }

    async fn download(download: Download) {
        let (writer_tx, mut writer_rx) = mpsc::channel(16);
        let (verifier_tx, mut verifier_rx) = mpsc::channel(16);

        let downloader = task::spawn(async move {
            let mut response = download.request.send().await.unwrap();

            while let Some(chunk) = response.chunk().await.unwrap() {
                if (writer_tx.send(chunk).await).is_err() {
                    break;
                }
            }
        });

        fs::create_dir_all(&download.download_options.install_location)
            .await
            .unwrap();

        let file_path = download
            .download_options
            .install_location
            .join(&download.file_name);

        let mut file = OpenOptions::new()
            .create(true)
            .truncate(true)
            .write(true)
            .open(&file_path)
            .await
            .unwrap();

        let md5_exists = download.md5.is_some();
        let writer = task::spawn(async move {
            while let Some(chunk) = writer_rx.recv().await {
                file.write_all(&chunk).await.unwrap();
                if md5_exists {
                    verifier_tx.send(chunk).await.unwrap();
                }
            }
        });

        let verifier = task::spawn(async move {
            let mut hasher = md5::Context::new();
            while let Some(chunk) = verifier_rx.recv().await {
                hasher.consume(&chunk);
            }
            hasher.compute()
        });

        downloader.await.unwrap();
        writer.await.unwrap();

        if let Some(md5) = download.md5 {
            let result = verifier.await.unwrap();
            println!("MD5: {:x}", result);
            if format!("{:x}", result) != md5 {
                println!("MD5 mismatch!");
            }
        }

        println!("Downloaded: {}", download.file_name);
    }
}
