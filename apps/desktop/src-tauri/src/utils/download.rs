use super::file::OpenWithDirs;
use crate::{common::result::Result, models::download::DownloadProgress};
use md5::{Digest, Md5};
use reqwest::{header::RANGE, RequestBuilder};
use std::{
    io::SeekFrom,
    path::PathBuf,
    sync::{
        atomic::{AtomicU64, Ordering},
        Arc,
    },
};
use tokio::{
    fs::{File, OpenOptions},
    io::{AsyncReadExt, AsyncSeekExt, AsyncWriteExt},
    pin,
    sync::mpsc,
    task,
};
use tokio_util::{bytes::Bytes, sync::CancellationToken};

pub async fn download_file(
    request: RequestBuilder,
    path: PathBuf,
    cancellation_token: CancellationToken,
    progress_tx: mpsc::Sender<DownloadProgress>,
    md5: Option<String>,
) -> Result<bool> {
    let mut file = OpenOptions::new()
        .create(true)
        .truncate(false)
        .write(true)
        .read(true)
        .open_with_dirs(&path)
        .await?;

    let file_size = file.metadata().await?.len();

    let hasher = if file_size > 0 && md5.is_some() {
        let mut buffer = vec![0; 65536];
        let mut context = Md5::new();

        loop {
            let bytes_read = file.read(&mut buffer).await?;
            if bytes_read == 0 {
                break;
            }
            context.update(&buffer[..bytes_read]);
        }

        context
    } else {
        file.seek(SeekFrom::End(0)).await?;
        Md5::new()
    };

    let (writer_tx, writer_rx) = mpsc::channel(16);
    let (verifier_tx, verifier_rx) = mpsc::channel(16);

    let total_written = Arc::new(AtomicU64::new(file_size));
    let total_downloaded = Arc::new(AtomicU64::new(file_size));

    let reporter = task::spawn(reporter(
        Arc::clone(&total_downloaded),
        Arc::clone(&total_written),
        progress_tx,
    ));
    let writer = task::spawn(writer(
        writer_rx,
        file,
        total_written,
        total_downloaded,
        if md5.is_some() {
            Some(verifier_tx)
        } else {
            None
        },
    ));
    let verifier = task::spawn(verifier(verifier_rx, hasher));
    let downloader = task::spawn(downloader(request, writer_tx, file_size));

    pin!(downloader);
    tokio::select! {
        _ = cancellation_token.cancelled() => {
            downloader.abort();
        }
        _ = &mut downloader => {},

    }

    writer.await?;
    let verifier_result = verifier.await?;
    reporter.abort();

    if cancellation_token.is_cancelled() {
        println!("Download cancelled.");
        return Ok(false);
    }

    if let Some(md5) = md5 {
        if verifier_result != md5 {
            println!("MD5 mismatch!");
        }
    }

    Ok(true)
}

async fn downloader(request: RequestBuilder, tx: mpsc::Sender<Bytes>, initial_size: u64) {
    let mut response = request
        .header(RANGE, format!("bytes={}-", initial_size))
        .send()
        .await
        .unwrap();

    while let Some(chunk) = response.chunk().await.unwrap() {
        if (tx.send(chunk).await).is_err() {
            break;
        }
    }
}

async fn writer(
    mut rx: mpsc::Receiver<Bytes>,
    mut file: File,
    total_written: Arc<AtomicU64>,
    total_downloaded: Arc<AtomicU64>,
    tx: Option<mpsc::Sender<Bytes>>,
) {
    while let Some(chunk) = rx.recv().await {
        total_downloaded.fetch_add(chunk.len() as u64, Ordering::Relaxed);
        file.write_all(&chunk).await.unwrap();
        total_written.fetch_add(chunk.len() as u64, Ordering::Relaxed);
        if let Some(tx) = &tx {
            tx.send(chunk).await.unwrap();
        }
    }
}

async fn verifier(mut rx: mpsc::Receiver<Bytes>, mut hasher: Md5) -> String {
    while let Some(chunk) = rx.recv().await {
        hasher.update(&chunk);
    }
    let result = hasher.finalize();
    format!("{:x}", result)
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
