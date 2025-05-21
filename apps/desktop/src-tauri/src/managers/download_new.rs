use std::{
    collections::VecDeque,
    path::Path,
    sync::{Arc, Mutex},
};

use tokio::{
    fs,
    io::AsyncWriteExt,
    sync::{mpsc, Notify},
};
use tokio_util::sync::CancellationToken;

use crate::{
    common::{download::process_download, result::Result},
    models::download::{Download, DownloadProgress},
};

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

    pub async fn enqueue_download(&self, download: Download) -> Result<()> {
        let manifest_path = download.path.join(".downloading").join("manifest.json");

        let download = if Path::new(&manifest_path).exists() {
            let manifest = fs::read_to_string(&manifest_path).await?;
            serde_json::from_str(&manifest)?
        } else {
            if let Some(parent) = manifest_path.parent() {
                fs::create_dir_all(parent).await?;
            }

            let json = serde_json::to_string_pretty(&download)?;
            let mut file = fs::File::create(&manifest_path).await?;
            file.write_all(json.as_bytes()).await?;

            download
        };

        println!("Enqueuing download: {:?}", download.game_id);
        let mut queue = self.queue.lock().unwrap();
        queue.push_back(download);
        self.queue_notifier.notify_one();
        Ok(())
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

                    let (progress_tx, mut progress_rx) = mpsc::channel::<DownloadProgress>(50);

                    let progress_agregator = tokio::spawn(async move {
                        while let Some(update) = progress_rx.recv().await {
                            println!("Downloaded chunk: {}", update.chunk_id);
                        }
                    });

                    let result = process_download(download, token.clone(), progress_tx).await;
                    progress_agregator.await.unwrap();
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
