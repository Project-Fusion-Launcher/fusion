use super::api::models::manifest::ChunkInfo;
use crate::{
    common::{result::Result, worker::WorkerPool},
    downloads::DownloadStrategy,
    models::download::*,
    storefronts::{epicgames::api::models::chunk::Chunk, get_epic_games},
    utils::file,
};
use async_trait::async_trait;
use reqwest::Url;
use std::sync::{
    atomic::{AtomicU64, Ordering},
    Arc,
};
use tokio::{select, sync::mpsc, task};
use tokio_mpmc::Queue;
use tokio_util::sync::CancellationToken;

pub(super) struct EpicGamesStrategy {}

type Guid = (u32, u32, u32, u32);

#[async_trait]
impl DownloadStrategy for EpicGamesStrategy {
    async fn download(
        &self,
        download: &mut Download,
        cancellation_token: CancellationToken,
        progress_tx: mpsc::Sender<DownloadProgress>,
    ) -> Result<()> {
        let manifest = Arc::new(
            get_epic_games()
                .read()
                .await
                .get_game_manifest(&download.game_id, &download.game_version_id)
                .await?,
        );

        let base_url = Arc::new(
            get_epic_games()
                .read()
                .await
                .get_cdn_url(&download.game_id)
                .await?,
        );

        let writer_queue: Arc<Queue<(Guid, Chunk)>> = Arc::new(Queue::new(24));
        let decoder_queue: Arc<Queue<(Guid, Vec<u8>)>> = Arc::new(Queue::new(24));

        let downloaders = 16;
        let dl_pool = WorkerPool::new(downloaders);

        let writers = 16;
        let writer_pool = WorkerPool::new(writers);

        let decoders = 16;
        let decoder_pool = WorkerPool::new(decoders);

        let http = reqwest::Client::new();

        let total_written = Arc::new(AtomicU64::new(0));
        let total_downloaded = Arc::new(AtomicU64::new(0));

        for _ in 0..writers {
            let writer_queue = Arc::clone(&writer_queue);
            let manifest = Arc::clone(&manifest);
            let download_path = download.path.clone();

            let total_written_clone = Arc::clone(&total_written);
            writer_pool
                .execute(move || async move {
                    loop {
                        match writer_queue.receive().await {
                            Ok(Some(data)) => {
                                let chunk_files = manifest.chunk_files(data.0);

                                //let mut written = 0;

                                for chunk_file in chunk_files {
                                    let file_path = download_path.join(&chunk_file.filename);

                                    for chunk_part in chunk_file.chunk_parts.iter() {
                                        if chunk_part.guid == data.0 {
                                            file::write_at(
                                                file_path.to_str().unwrap(),
                                                &data.1.data[chunk_part.offset as usize
                                                    ..(chunk_part.offset + chunk_part.size)
                                                        as usize],
                                                chunk_part.file_offset,
                                            )
                                            .await
                                            .unwrap();

                                            //written += chunk_part.size as u64;
                                        }
                                    }
                                }

                                //total_written_clone.fetch_add(written, Ordering::Relaxed);
                            }
                            Ok(None) => break,
                            Err(e) => eprintln!("Receive failed: {}", e),
                        }
                    }
                })
                .await?;
        }

        for _ in 0..decoders {
            let decoder_queue = Arc::clone(&decoder_queue);
            let writer_queue = Arc::clone(&writer_queue);
            let total_downloaded_clone = Arc::clone(&total_downloaded);

            decoder_pool
                .execute(move || async move {
                    loop {
                        match decoder_queue.receive().await {
                            Ok(Some(data)) => {
                                /*total_downloaded_clone
                                .fetch_add(data.1.len() as u64, Ordering::Relaxed);*/

                                let decoded_chunk = Chunk::new(data.1).unwrap();

                                if writer_queue.is_full() {
                                    println!("Writer queue is full.");
                                }
                                writer_queue.send((data.0, decoded_chunk)).await.unwrap();
                            }
                            Ok(None) => break,
                            Err(e) => eprintln!("Receive failed: {}", e),
                        }
                    }
                })
                .await?;
        }

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

        for chunk in manifest.cdl.elements.iter() {
            if cancellation_token.is_cancelled() {
                break;
            }

            let token = cancellation_token.clone();
            let decoder_queue = Arc::clone(&decoder_queue);
            let chunk = chunk.clone();
            let http = http.clone();
            let base_url = Arc::clone(&base_url);

            dl_pool
                .execute(move || download_chunk(http, base_url, chunk, token, decoder_queue))
                .await?;
        }

        dl_pool.shutdown().await;

        decoder_queue.close().await;
        decoder_pool.shutdown().await;

        writer_queue.close().await;
        writer_pool.shutdown().await;

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
    chunk: ChunkInfo,
    cancellation_token: CancellationToken,
    decoder_queue: Arc<Queue<(Guid, Vec<u8>)>>,
) -> Result<()> {
    let request = http.get(base_url.join(&chunk.path()).unwrap()).header(
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
                    let bytes = response.bytes().await?.to_vec();
                    if decoder_queue.is_full() {
                        println!("Decoder queue is full.");
                    }
                    decoder_queue.send((chunk.guid, bytes)).await.unwrap();

                }
                Err(e) => {
                    println!("Error downloading chunk: {:?}", e);
                }
            }
        }
    }

    Ok(())
}
