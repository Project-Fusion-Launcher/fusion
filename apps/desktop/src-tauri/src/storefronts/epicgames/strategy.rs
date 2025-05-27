use super::api::models::download_plan::{DownloadTask, WriteTask};
use crate::{
    common::{result::Result, worker::WorkerPool},
    downloads::DownloadStrategy,
    models::download::*,
    storefronts::{epicgames::api::models::chunk::Chunk, get_epic_games},
};
use async_trait::async_trait;
use reqwest::Url;
use std::{
    collections::HashMap,
    sync::{
        atomic::{AtomicU64, Ordering},
        Arc,
    },
};
use tokio::{
    fs::{self, OpenOptions},
    io::AsyncWriteExt,
    select,
    sync::{mpsc, Mutex, Notify},
    task,
};
use tokio_util::sync::CancellationToken;

pub(super) struct EpicGamesStrategy {}

#[async_trait]
impl DownloadStrategy for EpicGamesStrategy {
    async fn download(
        &self,
        download: &mut Download,
        cancellation_token: CancellationToken,
        progress_tx: mpsc::Sender<DownloadProgress>,
    ) -> Result<()> {
        let plan = get_epic_games()
            .read()
            .await
            .compute_download_plan(&download.game_id)
            .await?;

        let base_url = Arc::new(
            get_epic_games()
                .read()
                .await
                .get_cdn_url(&download.game_id)
                .await?,
        );

        //let writer_queue: Arc<Queue<Chunk>> = Arc::new(Queue::new(24));
        let (tx, mut rx) = mpsc::channel::<Vec<u8>>(32);

        let downloaders = 16;
        let dl_pool = WorkerPool::new(downloaders);

        let http = reqwest::Client::new();

        let total_written = Arc::new(AtomicU64::new(0));
        let total_downloaded = Arc::new(AtomicU64::new(0));

        let cancellation_token_clone = cancellation_token.clone();

        let chunk_map = Arc::new(Mutex::new(HashMap::new()));
        let notify = Arc::new(Notify::new());

        let chunk_map_clone = Arc::clone(&chunk_map);
        let notify_clone = Arc::clone(&notify);
        let receiver = task::spawn(async move {
            while let Some(data) = rx.recv().await {
                let chunk = Chunk::new(data).unwrap();
                chunk_map_clone
                    .lock()
                    .await
                    .insert(chunk.header.guid, chunk);
                notify_clone.notify_one();
            }
        });

        let mut tasks = plan.write_tasks;
        let download_path = download.path.clone();
        let writer = task::spawn(async move {
            let mut opened_file = None;
            while let Some(task) = tasks.pop_front() {
                match task {
                    WriteTask::Open { filename } => {
                        let path = download_path.join(&filename);
                        if !path.exists() {
                            fs::create_dir_all(path.parent().unwrap()).await.unwrap();
                        }
                        opened_file = Some(
                            OpenOptions::new()
                                .write(true)
                                .create(true)
                                .truncate(true)
                                .open(&path)
                                .await
                                .unwrap(),
                        );
                    }
                    WriteTask::Write {
                        chunk_guid,
                        remove_cache,
                        chunk_offset,
                        size,
                    } => loop {
                        let chunk = { chunk_map.lock().await.remove(&chunk_guid) };
                        if let Some(chunk) = chunk {
                            if let Some(file) = opened_file.as_mut() {
                                file.write_all(&chunk.data[chunk_offset..chunk_offset + size])
                                    .await
                                    .unwrap();
                            }

                            if !remove_cache {
                                chunk_map.lock().await.insert(chunk_guid, chunk);
                            }

                            break;
                        }

                        select! {
                            _ = notify.notified() => {}

                            _ = tokio::time::sleep(std::time::Duration::from_secs(1)) => {
                                //println!("Retrying to get chunk: {:?}", chunk_guid);
                            }
                        }
                    },
                    WriteTask::Close { sha1: _ } => {
                        let to_close = opened_file.take();
                        drop(to_close);
                    }
                };
            }
        });

        let reporter = task::spawn(async move {
            loop {
                let downloaded = total_downloaded.load(Ordering::Relaxed);
                let written = total_written.load(Ordering::Relaxed);
                if progress_tx
                    .send(DownloadProgress {
                        downloaded,
                        written,
                    })
                    .await
                    .is_err()
                {
                    eprintln!("Progress reporter channel closed.");
                }
                tokio::time::sleep(std::time::Duration::from_secs(1)).await;
            }
        });

        let start_time = std::time::Instant::now();

        for download_task in plan.download_tasks {
            if cancellation_token.is_cancelled() {
                break;
            }

            let token = cancellation_token.clone();
            let tx = tx.clone();
            let http = http.clone();
            let base_url = Arc::clone(&base_url);

            dl_pool
                .execute(move || download_chunk(http, base_url, download_task, token, tx))
                .await?;
        }

        drop(tx);

        dl_pool.shutdown().await;
        receiver.await?;
        writer.await?;
        reporter.abort();

        let elapsed_time = start_time.elapsed();
        download.completed = true;
        println!("Download completed in {:?}", elapsed_time);

        Ok(())
    }
}

async fn download_chunk(
    http: reqwest::Client,
    base_url: Arc<Url>,
    task: DownloadTask,
    cancellation_token: CancellationToken,
    writer_tx: mpsc::Sender<Vec<u8>>,
) -> Result<()> {
    let request = http.get(base_url.join(&task.chunk_path).unwrap()).header(
        "User-Agent",
        "EpicGamesLauncher/11.0.1-14907503+++Portal+Release-Live Windows/10.0.19041.1.256.64bit",
    );

    select! {
        biased;

        _ = cancellation_token.cancelled() => {
        }

        response = request.send() => {
            match response {
                Ok(response) => {
                    let bytes = response.bytes().await?;
                    writer_tx.send(bytes.into()).await.unwrap();
                }
                Err(e) => {
                    println!("Error downloading chunk: {:?}", e);
                }
            }
        }
    }

    Ok(())
}
