use crate::{
    models::{game::GameSource, payloads::DownloadPayload},
    storefronts::get_storefront,
    APP,
};
use reqwest::RequestBuilder;
use serde::Deserialize;
use std::{
    collections::VecDeque,
    path::PathBuf,
    sync::{
        atomic::{AtomicU64, Ordering},
        Arc, Mutex,
    },
    time::Duration,
};
use tauri::Emitter;
use tokio::{
    fs::{self, OpenOptions},
    io::AsyncWriteExt,
    sync::{mpsc, Notify},
    task, time,
};

pub struct Download {
    pub request: RequestBuilder,
    pub file_name: String,
    pub game_source: GameSource,
    pub game_id: String,
    pub game_title: String,
    pub md5: Option<String>,
    pub download_size: u64,
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
    pub fn init() -> Self {
        let manager = Self {
            queue: Arc::new(Mutex::new(VecDeque::new())),
            queue_notifier: Arc::new(Notify::new()),
        };

        manager.process_queue();
        manager
    }

    pub fn enqueue_download(&self, download: Download) {
        let payload = DownloadPayload {
            game_id: download.game_id.clone(),
            game_source: download.game_source.clone(),
            game_title: download.game_title.clone(),
            download_size: download.download_size,
            downloaded: 0,
        };

        self.queue.lock().unwrap().push_back(download);

        APP.get().unwrap().emit("download-queued", payload).unwrap();

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

                    let payload = DownloadPayload {
                        game_id: download.game_id.clone(),
                        game_source: download.game_source.clone(),
                        game_title: download.game_title.clone(),
                        download_size: download.download_size,
                        downloaded: download.download_size,
                    };

                    Self::download(download).await;

                    APP.get()
                        .unwrap()
                        .emit("download-finished", &payload)
                        .unwrap();

                    let result = get_storefront(&payload.game_source)
                        .read()
                        .await
                        .post_download(&payload.game_id, path, &file_name)
                        .await;

                    APP.get()
                        .unwrap()
                        .emit("download-installed", &payload)
                        .unwrap();

                    if let Err(e) = result {
                        println!("Error post-download: {}", e);
                    }
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
        let total_written = Arc::new(AtomicU64::new(0));
        let total_written_clone = total_written.clone();

        let progress_reporter = task::spawn(async move {
            let mut interval = time::interval(Duration::from_secs(1));
            let app_handle = APP.get().unwrap();
            loop {
                interval.tick().await;
                let written = total_written.load(Ordering::Relaxed);
                println!("Downloaded: {}", written);
                app_handle
                    .emit(
                        "download-progress",
                        DownloadPayload {
                            game_id: download.game_id.clone(),
                            game_source: download.game_source.clone(),
                            game_title: download.game_title.clone(),
                            download_size: download.download_size,
                            downloaded: written,
                        },
                    )
                    .unwrap();
            }
        });

        let writer = task::spawn(async move {
            while let Some(chunk) = writer_rx.recv().await {
                file.write_all(&chunk).await.unwrap();
                total_written_clone.fetch_add(chunk.len() as u64, Ordering::Relaxed);
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

        progress_reporter.abort();

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
