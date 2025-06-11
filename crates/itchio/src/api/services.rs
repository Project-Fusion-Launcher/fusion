use super::{endpoints, models::*};
use anyhow::{Result, anyhow};
use reqwest::{IntoUrl, RequestBuilder, header::AUTHORIZATION};
use serde::de::DeserializeOwned;

pub struct Services {
    http: reqwest::Client,
    api_key: String,
}

impl Services {
    pub async fn from_api_key(api_key: String) -> Result<Self> {
        let services = Self {
            http: reqwest::Client::new(),
            api_key,
        };

        let user = services.fetch_profile().await?;
        println!("[itch.io] Logged in as: {}", user.username);
        Ok(services)
    }

    pub async fn fetch_owned_keys(&self, page: u32) -> Result<OwnedKeys> {
        let url = endpoints::owned_keys(page);
        let response: OwnedKeysResponse = self.get(url).await?;
        match response {
            OwnedKeysResponse::OwnedKeys(owned_keys) => Ok(owned_keys),
            OwnedKeysResponse::Error(e) => Err(anyhow!("{}", e.errors.join("; "))),
        }
    }

    pub async fn fetch_game_uploads(
        &self,
        game_id: u32,
        download_key_id: u32,
    ) -> Result<Vec<Upload>> {
        let url = endpoints::game_uploads(game_id, download_key_id);
        let response: UploadsResponse = self.get(url).await?;
        match response {
            UploadsResponse::Uploads(uploads) => Ok(uploads),
            UploadsResponse::Error(e) => Err(anyhow!("{}", e.errors.join("; "))),
        }
    }

    pub async fn fetch_upload(&self, upload_id: u32, download_key_id: u32) -> Result<Upload> {
        let url = endpoints::upload(upload_id, download_key_id);
        let response: UploadResponse = self.get(&url).await?;
        match response {
            UploadResponse::Upload(upload) => Ok(*upload),
            UploadResponse::Error(e) => Err(anyhow!("{}", e.errors.join("; "))),
        }
    }

    pub async fn fetch_upload_scanned_archive(
        &self,
        upload_id: u32,
        download_key_id: u32,
    ) -> Result<ScannedArchive> {
        let url = endpoints::upload_scanned_archive(upload_id, download_key_id);
        let response: ScannedArchiveResponse = self.get(url).await?;
        match response {
            ScannedArchiveResponse::ScannedArchive(scanned_archive) => Ok(scanned_archive),
            ScannedArchiveResponse::Error(e) => Err(anyhow!("{}", e.errors.join("; "))),
        }
    }

    pub fn fetch_upload_download(&self, upload_id: u32, download_key_id: u32) -> RequestBuilder {
        let url = endpoints::upload_download(upload_id, download_key_id);
        self.http.get(url).header(AUTHORIZATION, &self.api_key)
    }

    async fn fetch_profile(&self) -> Result<User> {
        let url = endpoints::profile();
        let response: ProfileResponse = self.get(url).await?;
        match response {
            ProfileResponse::User(user) => Ok(user),
            ProfileResponse::Error(e) => Err(anyhow!("{}", e.errors.join("; "))),
        }
    }

    async fn get<D, U>(&self, url: U) -> Result<D>
    where
        D: DeserializeOwned,
        U: IntoUrl,
    {
        Ok(self
            .http
            .get(url)
            .header(AUTHORIZATION, &self.api_key)
            .send()
            .await?
            .json()
            .await?)
    }
}
