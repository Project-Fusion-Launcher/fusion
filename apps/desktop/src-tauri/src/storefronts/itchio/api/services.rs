use super::{endpoints, models::*};
use crate::common::result::Result;
use reqwest::{header::AUTHORIZATION, IntoUrl, RequestBuilder};
use serde::de::DeserializeOwned;

pub struct Services {
    pub http: reqwest::Client,
    pub api_key: String,
}

impl Services {
    pub fn new(api_key: String) -> Self {
        Self {
            http: reqwest::Client::new(),
            api_key,
        }
    }

    pub async fn fetch_owned_keys(&self, page: u32) -> Result<OwnedKeys> {
        let url = endpoints::owned_keys(page);
        self.get(url, &self.api_key).await
    }

    pub async fn fetch_game_uploads(
        &self,
        game_id: u32,
        download_key_id: u32,
    ) -> Result<Vec<Upload>> {
        let url = endpoints::game_uploads(game_id, download_key_id);
        let response: Uploads = self.get(url, &self.api_key).await?;
        Ok(response.uploads)
    }

    pub async fn fetch_upload(&self, upload_id: u32, download_key_id: u32) -> Result<Upload> {
        let url = endpoints::upload(upload_id, download_key_id);
        self.get(url, &self.api_key).await
    }

    pub async fn fetch_upload_scanned_archive(
        &self,
        upload_id: u32,
        download_key_id: u32,
    ) -> Result<ScannedArchive> {
        let url = endpoints::upload_scanned_archive(upload_id, download_key_id);
        let response: ScannedArchiveResponse = self.get(url, &self.api_key).await?;
        Ok(response.scanned_archive)
    }

    pub fn fetch_upload_download(&self, upload_id: u32, download_key_id: u32) -> RequestBuilder {
        let url = endpoints::upload_download(upload_id, download_key_id);
        self.http.get(url).header("Authorization", &self.api_key)
    }

    async fn get<D, U>(&self, url: U, api_key: &str) -> Result<D>
    where
        D: DeserializeOwned,
        U: IntoUrl,
    {
        Ok(self
            .http
            .get(url)
            .header(AUTHORIZATION, api_key)
            .send()
            .await?
            .json()
            .await?)
    }
}
