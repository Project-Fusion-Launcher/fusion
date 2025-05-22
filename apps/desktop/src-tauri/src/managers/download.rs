use crate::{
    common::{result::Result, worker::WorkerPool},
    models::{
        download::{Download, DownloadChunk, DownloadProgress},
        game::GameSource,
    },
    storefronts::get_storefront,
    util::file,
};
use std::{
    collections::VecDeque,
    sync::{Arc, Mutex},
};
use tauri::utils::acl::manifest;
use tokio::{
    fs::{self, OpenOptions},
    io::{AsyncSeekExt, AsyncWriteExt},
    select,
    sync::{mpsc, Notify},
};
use tokio_mpmc::Queue;
use tokio_util::sync::CancellationToken;
use wrapper_epicgames::{api::models::chunk, EpicGamesClient};

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

                    let result =
                        Self::process_download(&mut download, token.clone(), progress_tx).await;
                    progress_agregator.await.unwrap();
                    println!("Download result: {:?}", result);

                    if !download.completed {
                        let mut queue_lock = queue_clone.lock().unwrap();
                        queue_lock.push_front(download);
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

    async fn process_download(
        download: &mut Download,
        cancellation_token: CancellationToken,
        progress_tx: mpsc::Sender<DownloadProgress>,
    ) -> Result<()> {
        fs::create_dir_all(&download.path.join(".downloading")).await?;

        let manifest = Arc::new(
            get_storefront(&download.game_source)
                .read()
                .await
                .game_manifest(&download.game_id, &download.game_version_id)
                .await?,
        );

        let writer_queue: Arc<Queue<(u128, Vec<u8>)>> = Arc::new(Queue::new(16));

        let writers = 8;
        let writer_pool = WorkerPool::new(writers);
        let dl_pool = WorkerPool::new(16);
        let game_source = download.game_source;
        let http = Arc::new(reqwest::Client::new());

        for _ in 0..writers {
            let writer_queue = Arc::clone(&writer_queue);
            let manifest = Arc::clone(&manifest);
            let download_path = download.path.clone();
            writer_pool
                .execute(move || async move {
                    loop {
                        match writer_queue.receive().await {
                            Ok(Some(data)) => {
                                let decoded_chunk = EpicGamesClient::decode_chunk(&data.1).unwrap();
                                let chunk_files = manifest.chunk_files(data.0);

                                for chunk_file in chunk_files {
                                    let file_path = download_path.join(chunk_file.filename);

                                    for chunk_part in chunk_file.chunk_parts {
                                        file::write_at(
                                            file_path.to_str().unwrap(),
                                            &decoded_chunk.data[chunk_part.chunk_offset as usize
                                                ..(chunk_part.chunk_offset + chunk_part.size)
                                                    as usize],
                                            chunk_part.file_offset,
                                        )
                                        .await;
                                    }
                                }
                            }
                            Ok(None) => break,
                            Err(e) => eprintln!("Receive failed: {}", e),
                        }
                    }
                })
                .await?;
        }

        let start_time = std::time::Instant::now();

        for chunk in &manifest.chunks {
            if cancellation_token.is_cancelled() {
                break;
            }

            let token = cancellation_token.clone();
            let http = Arc::clone(&http);
            let writer_queue = Arc::clone(&writer_queue);
            let chunk = chunk.clone();

            dl_pool
                .execute(move || {
                    download_chunk(http.clone(), chunk, game_source, token, writer_queue)
                })
                .await?;
        }

        dl_pool.shutdown().await;
        writer_queue.close().await;
        writer_pool.shutdown().await;

        let elapsed_time = start_time.elapsed();
        download.completed = true;
        println!("Download completed in {:?}", elapsed_time);

        Ok(())
    }
}

async fn download_chunk(
    http: Arc<reqwest::Client>,
    chunk: DownloadChunk,
    game_source: GameSource,
    cancellation_token: CancellationToken,
    writer_queue: Arc<Queue<(u128, Vec<u8>)>>,
) -> Result<()> {
    let request = get_storefront(&game_source)
        .read()
        .await
        .chunk_request(&http, &chunk.url)
        .await?;

    select! {
        biased;

        _ = cancellation_token.cancelled() => {
            println!("Downloader cancelled");
        }

        response = request.send() => {
            match response {
                Ok(response) => {
                    let bytes = response.bytes().await?.to_vec();
                    writer_queue.send((chunk.id, bytes)).await.unwrap();

                }
                Err(e) => {
                    println!("Error downloading chunk: {:?}", e);
                }
            }
        }
    }

    Ok(())
}
