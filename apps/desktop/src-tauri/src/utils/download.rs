use super::file::OpenWithDirs;
use crate::{common::result::Result, models::download::Download};
use md5::{Digest, Md5};
use reqwest::{header::RANGE, RequestBuilder};
use std::{io::SeekFrom, path::PathBuf, sync::Arc};
use tokio::{
    fs::{File, OpenOptions},
    io::{AsyncReadExt, AsyncSeekExt, AsyncWriteExt},
    pin,
    sync::mpsc,
    task,
};
use tokio_util::{bytes::Bytes, sync::CancellationToken};

pub async fn download_file(
    path: PathBuf,
    request: RequestBuilder,
    cancellation_token: CancellationToken,
    md5: Option<String>,
    download: Option<Arc<Download>>,
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

    let writer = task::spawn(writer(
        writer_rx,
        file,
        download,
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
    download: Option<Arc<Download>>,
    tx: Option<mpsc::Sender<Bytes>>,
) {
    while let Some(chunk) = rx.recv().await {
        if let Some(download) = &download {
            download.add_downloaded(chunk.len() as u64);
        }

        file.write_all(&chunk).await.unwrap();

        if let Some(download) = &download {
            download.add_written(chunk.len() as u64);
        }

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
