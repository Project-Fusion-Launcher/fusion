use crate::{
    common::result::Result,
    models::download::{Download, DownloadProgress},
};
use async_trait::async_trait;
use tokio::sync::mpsc;
use tokio_util::sync::CancellationToken;

#[async_trait]
pub trait DownloadStrategy: Send + Sync {
    async fn download(
        &self,
        download: &mut Download,
        cancellation_token: CancellationToken,
        progress_tx: mpsc::Sender<DownloadProgress>,
    ) -> Result<()>;
}
