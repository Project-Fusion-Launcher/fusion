use crate::{
    common::result::Result,
    models::download::Download,
    storefronts::{get_itchio, DownloadStrategy},
    utils::download::download_file,
};
use async_trait::async_trait;
use reqwest::RequestBuilder;
use std::sync::Arc;
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
        download: Arc<Download>,
        cancellation_token: CancellationToken,
    ) -> Result<bool> {
        let download_info = get_itchio()
            .read()
            .await
            .fetch_download_info(&download)
            .await?;

        let path = download.path.join(download_info.filename);

        let result = download_file(
            path,
            download_info.request,
            cancellation_token,
            download_info.md5,
            Some(download),
        )
        .await?;

        Ok(result)
    }
}
