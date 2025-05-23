use crate::{
    common::result::Result,
    downloads::DownloadStrategy,
    models::download::{Download, DownloadProgress},
    storefronts::get_itchio,
};
use async_trait::async_trait;
use reqwest::RequestBuilder;
use std::sync::{
    atomic::{AtomicU64, Ordering},
    Arc,
};
use tokio::{
    fs::{self, OpenOptions},
    io::AsyncWriteExt,
    sync::mpsc,
    task,
};
use tokio_util::sync::CancellationToken;

pub(super) struct ItchioStrategy {}

#[async_trait]
impl DownloadStrategy for ItchioStrategy {
    async fn download(
        &self,
        download: &mut Download,
        _cancellation_token: CancellationToken,
        _progress_tx: mpsc::Sender<DownloadProgress>,
    ) -> Result<()> {
        let download_info = get_itchio()
            .read()
            .await
            .fetch_download_info(download)
            .await?;

        let (writer_tx, mut writer_rx) = mpsc::channel(16);
        let (verifier_tx, mut verifier_rx) = mpsc::channel(16);

        let downloader = task::spawn(async move {
            let mut response = download_info.request.send().await.unwrap();

            while let Some(chunk) = response.chunk().await.unwrap() {
                if (writer_tx.send(chunk).await).is_err() {
                    break;
                }
            }
        });

        fs::create_dir_all(&download.path).await.unwrap();

        let file_path = download.path.join(&download_info.filename);

        let mut file = OpenOptions::new()
            .create(true)
            .truncate(true)
            .write(true)
            .open(&file_path)
            .await
            .unwrap();

        let md5_exists = download_info.md5.is_some();
        let total_written = Arc::new(AtomicU64::new(0));
        let total_written_clone = total_written.clone();

        let writer = task::spawn(async move {
            while let Some(chunk) = writer_rx.recv().await {
                file.write_all(&chunk).await.unwrap();
                total_written_clone.fetch_add(chunk.len() as u64, Ordering::Relaxed);
                if md5_exists {
                    verifier_tx.send(chunk).await.unwrap();
                }
            }
        });

        let verifier = task::spawn(async move {
            let mut hasher = md5::Context::new();
            while let Some(chunk) = verifier_rx.recv().await {
                hasher.consume(&chunk);
            }
            hasher.compute()
        });

        downloader.await.unwrap();
        writer.await.unwrap();
        let verifier_result = verifier.await.unwrap();

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

pub(super) struct ItchioDownload {
    pub request: RequestBuilder,
    pub filename: String,
    pub md5: Option<String>,
}
