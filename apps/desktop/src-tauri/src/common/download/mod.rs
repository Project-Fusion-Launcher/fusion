use super::{result::Result, worker::WorkerPool};
use crate::{
    models::{
        download::{Download, DownloadChunk, DownloadProgress},
        game::GameSource,
    },
    storefronts::get_storefront,
};
use std::{
    path::Path,
    sync::{
        atomic::{AtomicU64, Ordering},
        Arc,
    },
};
use tokio::{
    fs::{self, OpenOptions},
    io::AsyncWriteExt,
    select,
    sync::mpsc,
    task::{self, JoinHandle},
};
use tokio_util::sync::CancellationToken;

pub async fn process_download(
    download: Download,
    cancellation_token: CancellationToken,
    progress_tx: mpsc::Sender<DownloadProgress>,
) -> Result<()> {
    /*let pool = WorkerPool::new(16);
    fs::create_dir_all(&download.path.join(".downloading")).await?;

    for chunk in download.chunks {
        if cancellation_token.is_cancelled() {
            break;
        }

        let token = cancellation_token.clone();
        let path = download.path.clone();
        let progress_tx = progress_tx.clone();
        pool.execute(move || download_chunk(path, chunk, download.game_source, token, progress_tx))
            .await?;
    }

    pool.shutdown().await;*/

    Ok(())
}

pub async fn download_chunk<P: AsRef<Path>>(
    download_path: P,
    chunk: DownloadChunk,
    game_source: GameSource,
    cancellation_token: CancellationToken,
    progress_tx: mpsc::Sender<DownloadProgress>,
) -> Result<()> {
    if cancellation_token.is_cancelled() {
        return Ok(());
    }

    /*let (writer_tx, mut writer_rx) = mpsc::channel(10);

    let file_path = download_path
        .as_ref()
        .join(".downloading")
        .join(chunk.id.to_string());

    let mut file = OpenOptions::new()
        .create(true)
        .truncate(true)
        .write(true)
        .open(&file_path)
        .await?;

    let total_written = Arc::new(AtomicU64::new(0));

    let cancellation_token_clone = cancellation_token.clone();
    let downloader: JoinHandle<Result<()>> = task::spawn(async move {
        let mut response = get_storefront(&game_source)
            .read()
            .await
            .chunk_request(&chunk.url)
            .await?
            .send()
            .await?;

        loop {
            select! {
                biased;

                _ = cancellation_token_clone.cancelled() => {
                    println!("Downloader cancelled");
                    break;
                }

                chunk_result = response.chunk() => {
                    match chunk_result? {
                        Some(data) => {
                            if writer_tx.send(data).await.is_err() {
                                return Err("Failed to send data to writer")?;
                            }
                        }
                        None => break,
                    }
                }
            }
        }

        drop(writer_tx);
        Ok(())
    });

    let total_written_clone = total_written.clone();
    let writer: JoinHandle<Result<()>> = task::spawn(async move {
        while let Some(data) = writer_rx.recv().await {
            file.write_all(&data).await?;
            total_written_clone.fetch_add(data.len() as u64, Ordering::Relaxed);
        }

        Ok(())
    });

    downloader.await??;
    writer.await??;

    let total_written = total_written.load(Ordering::Relaxed);
    if total_written == chunk.compressed_size {
        get_storefront(&game_source)
            .read()
            .await
            .process_chunk(file_path)
            .await?;

        progress_tx
            .send(DownloadProgress {
                chunk_id: chunk.id,
                completed: true,
            })
            .await
            .unwrap();

        println!("Downloaded chunk {}", chunk.id);
    } else if cancellation_token.is_cancelled() {
        println!("Chunk download cancelled");
    } else {
        println!(
            "Chunk size mismatch: expected {}, got {}",
            chunk.compressed_size, total_written
        );
        Err("Chunk size mismatch")?;
    }*/

    Ok(())
}
