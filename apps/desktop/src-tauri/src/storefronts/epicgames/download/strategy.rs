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
use std::{collections::VecDeque, sync::Arc};
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
        download: Arc<Download>,
        cancellation_token: CancellationToken,
    ) -> Result<bool> {
        let instant = std::time::Instant::now();
        let plan = get_epic_games()
            .read()
            .await
            .compute_download_plan(&download.game.id)
            .await?;
        let elapsed = instant.elapsed();
        println!("Download plan computed in {:?}", elapsed);

        let base_url = Arc::new(
            get_epic_games()
                .read()
                .await
                .get_cdn_url(&download.game.id)
                .await?,
        );

        let downloaders = 16;
        let (tx, rx) = mpsc::channel::<Vec<u8>>(downloaders * 2);
        let dl_pool = WorkerPool::new(downloaders);

        let http = reqwest::Client::new();

        let chunk_map: ShardMap<Guid, Chunk> = ShardMap::new();
        let notify = Arc::new(Notify::new());

        let decoder = task::spawn(decoder(
            rx,
            Arc::clone(&download),
            chunk_map.clone(),
            Arc::clone(&notify),
        ));
        let writer = task::spawn(writer(
            plan.write_tasks,
            download,
            cancellation_token.clone(),
            chunk_map,
            notify,
        ));

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

        let elapsed_time = start_time.elapsed();
        println!("Download completed in {:?}", elapsed_time);

        Ok(true)
    }
}

async fn decoder(
    mut rx: mpsc::Receiver<Vec<u8>>,
    download: Arc<Download>,
    chunk_map: ShardMap<Guid, Chunk>,
    notify: Arc<Notify>,
) {
    while let Some(data) = rx.recv().await {
        download.add_downloaded(data.len() as u64);
        let chunk = Chunk::new(data).unwrap();
        chunk_map.insert(chunk.header.guid, chunk).await;
        notify.notify_one();
    }
}

async fn writer(
    mut tasks: VecDeque<WriteTask>,
    download: Arc<Download>,
    cancellation_token: CancellationToken,
    chunk_map: ShardMap<Guid, Chunk>,
    notify: Arc<Notify>,
) {
    let mut opened_file = None;
    while let Some(task) = tasks.pop_front() {
        match task {
            WriteTask::Open { filename } => {
                let path = download.path.join(&filename);

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
                        download.add_written(size as u64);
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
