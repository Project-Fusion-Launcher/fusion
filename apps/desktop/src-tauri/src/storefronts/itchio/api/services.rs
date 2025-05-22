use super::{endpoints, models::*};
use crate::{common::result::Result, storefronts::itchio::utils};

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
        utils::get(&self.http, url, &self.api_key).await
    }

    pub async fn fetch_game_uploads(
        &self,
        game_id: u32,
        download_key_id: u32,
    ) -> Result<Vec<Upload>> {
        let url = endpoints::game_uploads(game_id, download_key_id);
        let response: Uploads = utils::get(&self.http, url, &self.api_key).await?;
        Ok(response.uploads)
    }

    pub async fn fetch_game_upload(&self, upload_id: u32, download_key_id: u32) -> Result<Upload> {
        let url = endpoints::game_upload(upload_id, download_key_id);
        utils::get(&self.http, url, &self.api_key).await
    }

    pub async fn fetch_upload_scanned_archive(
        &self,
        upload_id: u32,
        download_key_id: u32,
    ) -> Result<ScannedArchive> {
        let url = endpoints::upload_scanned_archive(upload_id, download_key_id);
        let response: ScannedArchiveResponse = utils::get(&self.http, url, &self.api_key).await?;
        Ok(response.scanned_archive)
    }
}
