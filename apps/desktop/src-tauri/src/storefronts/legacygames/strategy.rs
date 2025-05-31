use crate::{
    common::result::Result,
    models::download::{Download, DownloadProgress},
    storefronts::{get_legacy_games, DownloadStrategy},
    utils::download::download_file,
};
use async_trait::async_trait;
use reqwest::RequestBuilder;
use tokio::sync::mpsc;
use tokio_util::sync::CancellationToken;

pub(super) struct LegacyGamesDownload {
    pub request: RequestBuilder,
    pub filename: String,
    pub md5: Option<String>,
}

pub(super) struct LegacyGamesStrategy {}

#[async_trait]
impl DownloadStrategy for LegacyGamesStrategy {
    async fn start(
        &self,
        download: &mut Download,
        cancellation_token: CancellationToken,
        progress_tx: mpsc::Sender<DownloadProgress>,
    ) -> Result<()> {
        let download_info = get_legacy_games()
            .read()
            .await
            .fetch_download_info(download)
            .await?;

        let path = download.path.join(download_info.filename);

        let result = download_file(
            download_info.request,
            path,
            cancellation_token,
            progress_tx,
            download_info.md5,
        )
        .await?;

        download.completed = result;
        Ok(())
    }
}
