use crate::{
    common::{result::Result, worker::WorkerPool},
    downloads::DownloadStrategy,
    models::download::*,
    storefronts::get_storefront,
    utils::file,
};
use async_trait::async_trait;
use std::sync::Arc;
use tokio::{select, sync::mpsc};
use tokio_mpmc::Queue;
use tokio_util::sync::CancellationToken;
use wrapper_epicgames::EpicGamesClient;

pub(super) struct EpicGamesStrategy {}

#[async_trait]
impl DownloadStrategy for EpicGamesStrategy {
    async fn download(
        &self,
        download: &mut Download,
        cancellation_token: CancellationToken,
        _progress_tx: mpsc::Sender<DownloadProgress>,
    ) -> Result<()> {
        let manifest = Arc::new(
            get_storefront(&download.game_source)
                .read()
                .await
                .game_manifest(&download.game_id, &download.game_version_id)
                .await?,
        );

        let writer_queue: Arc<Queue<(u128, Vec<u8>)>> = Arc::new(Queue::new(24));

        let writers = 8;
        let writer_pool = WorkerPool::new(writers);
        let dl_pool = WorkerPool::new(16);
        let http = reqwest::Client::new();

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
                                        .await
                                        .unwrap();
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
            let writer_queue = Arc::clone(&writer_queue);
            let chunk = chunk.clone();
            let http = http.clone();

            dl_pool
                .execute(move || download_chunk(http, chunk, token, writer_queue))
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
    http: reqwest::Client,
    chunk: DownloadChunk,
    cancellation_token: CancellationToken,
    writer_queue: Arc<Queue<(u128, Vec<u8>)>>,
) -> Result<()> {
    let request = http.get(&chunk.url).header(
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
