use crate::{
    common::{result::Result, worker::WorkerPool},
    models::download::*,
    storefronts::{
        epicgames::{
            api::{models::*, Guid, USER_AGENT},
            download::download_plan::*,
        },
        get_epic_games, DownloadStrategy,
    },
    utils::file::OpenWithDirs,
};
use async_trait::async_trait;
use reqwest::{header, RequestBuilder};
use std::{
    collections::VecDeque,
    path::PathBuf,
    sync::{
        atomic::{AtomicU64, Ordering},
        Arc,
    },
};
use tokio::{
    fs::OpenOptions,
    io::AsyncWriteExt,
    select,
    sync::{mpsc, Notify},
    task,
};
use tokio_util::sync::CancellationToken;
use whirlwind::ShardMap;

pub struct EpicGamesStrategy {}

#[async_trait]
impl DownloadStrategy for EpicGamesStrategy {
    async fn start(
        &self,
        download: &mut Download,
        cancellation_token: CancellationToken,
        progress_tx: mpsc::Sender<DownloadProgress>,
    ) -> Result<()> {
        let instant = std::time::Instant::now();
        let plan = get_epic_games()
            .read()
            .await
            .compute_download_plan(&download.game_id)
            .await?;
        let elapsed = instant.elapsed();
        println!("Download plan computed in {:?}", elapsed);

        let base_url = Arc::new(
            get_epic_games()
                .read()
                .await
                .get_cdn_url(&download.game_id)
                .await?,
        );

        let downloaders = 16;
        let (tx, rx) = mpsc::channel::<Vec<u8>>(downloaders * 2);
        let dl_pool = WorkerPool::new(downloaders);

        let http = reqwest::Client::new();

        let total_written = Arc::new(AtomicU64::new(0));
        let total_downloaded = Arc::new(AtomicU64::new(0));

        let chunk_map: ShardMap<Guid, Chunk> = ShardMap::new();
        let notify = Arc::new(Notify::new());

        let decoder = task::spawn(decoder(
            rx,
            Arc::clone(&total_downloaded),
            chunk_map.clone(),
            Arc::clone(&notify),
        ));
        let writer = task::spawn(writer(
            plan.write_tasks,
            Arc::clone(&total_written),
            download.path.clone(),
            cancellation_token.clone(),
            chunk_map,
            notify,
        ));
        let reporter = task::spawn(reporter(total_downloaded, total_written, progress_tx));

        let start_time = std::time::Instant::now();

        for download_task in plan.download_tasks {
            if cancellation_token.is_cancelled() {
                break;
            }

            let token = cancellation_token.clone();
            let tx = tx.clone();
            let request = http
                .get(base_url.join(&download_task.chunk_path).unwrap())
                .header(header::USER_AGENT, USER_AGENT);

            dl_pool
                .execute(move || downloader(request, token, tx))
                .await?;
        }

        drop(tx);

        dl_pool.shutdown().await;
        decoder.await?;
        writer.await?;
        reporter.abort();

        let elapsed_time = start_time.elapsed();
        download.completed = true;
        println!("Download completed in {:?}", elapsed_time);

        Ok(())
    }
}

async fn decoder(
    mut rx: mpsc::Receiver<Vec<u8>>,
    total_downloaded: Arc<AtomicU64>,
    chunk_map: ShardMap<Guid, Chunk>,
    notify: Arc<Notify>,
) {
    while let Some(data) = rx.recv().await {
        total_downloaded.fetch_add(data.len() as u64, Ordering::Relaxed);
        let chunk = Chunk::new(data).unwrap();
        chunk_map.insert(chunk.header.guid, chunk).await;
        notify.notify_one();
    }
}

async fn writer(
    mut tasks: VecDeque<WriteTask>,
    total_written: Arc<AtomicU64>,
    download_path: PathBuf,
    cancellation_token: CancellationToken,
    chunk_map: ShardMap<Guid, Chunk>,
    notify: Arc<Notify>,
) {
    let mut opened_file = None;
    while let Some(task) = tasks.pop_front() {
        match task {
            WriteTask::Open { filename } => {
                let path = download_path.join(&filename);

                opened_file = Some(
                    OpenOptions::new()
                        .write(true)
                        .create(true)
                        .truncate(true)
                        .open_with_dirs(&path)
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
                if cancellation_token.is_cancelled() {
                    break;
                }
                let chunk = { chunk_map.remove(&chunk_guid).await };
                if let Some(chunk) = chunk {
                    if let Some(file) = opened_file.as_mut() {
                        file.write_all(&chunk.data[chunk_offset..chunk_offset + size])
                            .await
                            .unwrap();
                        total_written.fetch_add(size as u64, Ordering::Relaxed);
                    }

                    if !remove_cache {
                        chunk_map.insert(chunk_guid, chunk).await;
                    }

                    break;
                }

                select! {
                    _ = notify.notified() => {}
                    _ = tokio::time::sleep(std::time::Duration::from_secs(1)) => {}
                }
            },
            WriteTask::Close { sha1: _ } => {
                let to_close = opened_file.take();
                drop(to_close);
            }
        };
    }
}

async fn reporter(
    total_downloaded: Arc<AtomicU64>,
    total_written: Arc<AtomicU64>,
    tx: mpsc::Sender<DownloadProgress>,
) {
    loop {
        let downloaded = total_downloaded.load(Ordering::Relaxed);
        let written = total_written.load(Ordering::Relaxed);
        if tx
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
}

async fn downloader(
    request: RequestBuilder,
    cancellation_token: CancellationToken,
    tx: mpsc::Sender<Vec<u8>>,
) -> Result<()> {
    select! {
        biased;
        response = request.send() => {
            match response {
                Ok(response) => {
                    let bytes = response.bytes().await?;
                    tx.send(bytes.into()).await.unwrap();
                }
                Err(e) => {
                    eprintln!("Error downloading chunk: {:?}", e);
                }
            }
        }
        _ = cancellation_token.cancelled() => {}
    }

    Ok(())
}
