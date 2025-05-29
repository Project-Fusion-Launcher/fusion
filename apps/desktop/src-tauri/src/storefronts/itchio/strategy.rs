use crate::{
    common::result::Result,
    models::download::{Download, DownloadProgress},
    storefronts::{get_itchio, DownloadStrategy},
};
use async_trait::async_trait;
use md5::{Digest, Md5};
use reqwest::{header::RANGE, RequestBuilder};
use std::sync::{
    atomic::{AtomicU64, Ordering},
    Arc,
};
use tokio::{
    fs::{self, OpenOptions},
    io::{AsyncReadExt, AsyncWriteExt},
    pin,
    sync::mpsc,
    task,
};
use tokio_util::sync::CancellationToken;

pub(super) struct ItchioDownload {
    pub request: RequestBuilder,
    pub filename: String,
    pub md5: Option<String>,
}

pub(super) struct ItchioStrategy {}

#[async_trait]
impl DownloadStrategy for ItchioStrategy {
    async fn start(
        &self,
        download: &mut Download,
        cancellation_token: CancellationToken,
        progress_tx: mpsc::Sender<DownloadProgress>,
    ) -> Result<()> {
        let download_info = get_itchio()
            .read()
            .await
            .fetch_download_info(download)
            .await?;

        fs::create_dir_all(&download.path).await.unwrap();

        let file_path = download.path.join(&download_info.filename);
        let mut file = OpenOptions::new()
            .create(true)
            .append(true)
            .write(true)
            .open(&file_path)
            .await
            .unwrap();

        let file_size = file.metadata().await?.len();

        let mut hasher = if file_size > 0 && download_info.md5.is_some() {
            let mut buffer = vec![0; 65536];
            let mut file_clone = OpenOptions::new()
                .read(true)
                .open(&file_path)
                .await
                .unwrap();
            let mut context = Md5::new();

            while let Ok(bytes_read) = file_clone.read(&mut buffer).await {
                if bytes_read == 0 {
                    break;
                }
                context.update(&buffer[..bytes_read]);
            }

            context
        } else {
            Md5::new()
        };
        let (writer_tx, mut writer_rx) = mpsc::channel(16);
        let (verifier_tx, mut verifier_rx) = mpsc::channel(16);

        let downloader = task::spawn(async move {
            let mut response = download_info
                .request
                .header(RANGE, format!("bytes={}-", file_size))
                .send()
                .await
                .unwrap();

            while let Some(chunk) = response.chunk().await.unwrap() {
                if (writer_tx.send(chunk).await).is_err() {
                    break;
                }
            }
        });

        let md5_exists = download_info.md5.is_some();
        let total_written = Arc::new(AtomicU64::new(file_size));
        let total_downloaded = Arc::new(AtomicU64::new(file_size));

        let total_written_clone = Arc::clone(&total_written);
        let total_downloaded_clone = Arc::clone(&total_downloaded);

        let writer = task::spawn(async move {
            while let Some(chunk) = writer_rx.recv().await {
                total_downloaded_clone.fetch_add(chunk.len() as u64, Ordering::Relaxed);
                file.write_all(&chunk).await.unwrap();
                total_written_clone.fetch_add(chunk.len() as u64, Ordering::Relaxed);
                if md5_exists {
                    verifier_tx.send(chunk).await.unwrap();
                }
            }
        });

        let verifier = task::spawn(async move {
            while let Some(chunk) = verifier_rx.recv().await {
                hasher.update(&chunk);
            }
            hasher.finalize()
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
            return Ok(());
        }

        if let Some(md5) = download_info.md5 {
            println!("MD5: {:x}", verifier_result);
            if format!("{:x}", verifier_result) != md5 {
                println!("MD5 mismatch!");
            }
        }

        download.completed = true;

        Ok(())
    }
}
