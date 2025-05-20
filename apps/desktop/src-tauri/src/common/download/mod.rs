use super::{result::Result, worker::WorkerPool};
use crate::models::download::{Download, DownloadFile};
use std::{io::SeekFrom, path::Path};
use tokio::{
    fs::{self, OpenOptions},
    io::{AsyncSeekExt, AsyncWriteExt},
    select,
    sync::mpsc,
    task::{self, JoinHandle},
};
use tokio_util::sync::CancellationToken;

pub async fn process_download(
    download: Download,
    cancellation_token: CancellationToken,
) -> Result<()> {
    //let pool = WorkerPool::new(2);
    fs::create_dir_all(&download.path).await?;

    /*for mut file in download.files {
        if cancellation_token.is_cancelled() {
            println!("Download cancelled");
            break;
        }
        println!("Processing file: {}", file.filename);
        file.filename = download
            .path
            .join(&file.filename)
            .to_string_lossy()
            .to_string();
        let token = cancellation_token.clone();
        pool.execute(move || process_file(file, token)).await?;
    }

    pool.shutdown().await;*/

    Ok(())
}

pub async fn process_file(
    download_file: DownloadFile,
    cancellation_token: CancellationToken,
) -> Result<()> {
    /*for chunk in download_file.chunks {
        if cancellation_token.is_cancelled() {
            println!("Download cancelled");
            break;
        }
        println!("Downloading file: {}", download_file.filename);
        download_chunk(&download_file.filename, chunk, cancellation_token.clone()).await?;
    }*/

    Ok(())
}

pub async fn download_chunk<P: AsRef<Path>>(
    file_path: P,
    //chunk: DownloadFileChunk,
    cancellation_token: CancellationToken,
) -> Result<()> {
    /*if cancellation_token.is_cancelled() {
        println!("Download cancelled");
        return Ok(());
    }

    let (writer_tx, mut writer_rx) = mpsc::channel(16);

    let downloader: JoinHandle<Result<()>> = task::spawn(async move {
        let mut response = chunk.request.send().await?;

        loop {
            select! {
                biased;

                _ = cancellation_token.cancelled() => {
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

    let mut file = OpenOptions::new()
        .truncate(false)
        .write(true)
        .open(&file_path)
        .await?;

    file.seek(SeekFrom::Start(chunk.offset)).await?;

    let write: JoinHandle<Result<()>> = task::spawn(async move {
        while let Some(data) = writer_rx.recv().await {
            file.write_all(&data).await?;
        }

        Ok(())
    });

    downloader.await??;
    write.await??; */

    Ok(())
}
