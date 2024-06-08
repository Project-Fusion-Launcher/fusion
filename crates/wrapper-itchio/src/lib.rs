mod api;
mod tests;

use api::models::Build;
use api::models::BuildResponse;
use api::models::Builds;
use api::models::Collection;
use api::models::CollectionGames;
use api::models::CollectionsResponse;
use api::models::OwnedKeys;
use api::models::Upload;
use api::models::UploadResponse;
use api::models::Uploads;
use serde::de::DeserializeOwned;

pub struct ItchioClient {
    api_key: String,
    http: reqwest::Client,
}

impl ItchioClient {
    pub fn new<S: AsRef<str>>(api_key: S) -> Self {
        Self {
            api_key: api_key.as_ref().to_string(),
            http: reqwest::Client::new(),
        }
    }

    pub async fn fetch_owned_keys(&self, page: u32) -> Result<OwnedKeys, reqwest::Error> {
        self.make_get_request(&api::endpoints::owned_keys(page))
            .await
    }

    pub async fn fetch_game_uploads(
        &self,
        game_id: u32,
        download_key_id: u32,
    ) -> Result<Uploads, reqwest::Error> {
        self.make_get_request(&api::endpoints::uploads(game_id, download_key_id))
            .await
    }

    pub async fn fetch_game_upload(
        &self,
        upload_id: u32,
        download_key_id: u32,
    ) -> Result<Upload, reqwest::Error> {
        let response: UploadResponse = self
            .make_get_request(&api::endpoints::upload(upload_id, download_key_id))
            .await?;
        Ok(response.upload)
    }

    pub async fn fetch_upload_builds(
        &self,
        upload_id: u32,
        download_key_id: u32,
    ) -> Result<Builds, reqwest::Error> {
        self.make_get_request(&api::endpoints::builds(upload_id, download_key_id))
            .await
    }

    pub async fn fetch_upload_build(
        &self,
        build_id: u32,
        download_key_id: u32,
    ) -> Result<Build, reqwest::Error> {
        let response: BuildResponse = self
            .make_get_request(&api::endpoints::build(build_id, download_key_id))
            .await?;
        Ok(response.build)
    }

    pub async fn fetch_collections(&self) -> Result<Vec<Collection>, reqwest::Error> {
        let response: CollectionsResponse = self
            .make_get_request(&api::endpoints::collections())
            .await?;
        Ok(response.collections)
    }

    pub async fn fetch_collection_games(
        &self,
        collection_id: u32,
        page: u32,
    ) -> Result<CollectionGames, reqwest::Error> {
        self.make_get_request(&api::endpoints::collection_games(collection_id, page))
            .await
    }

    async fn make_get_request<T: DeserializeOwned>(&self, url: &str) -> Result<T, reqwest::Error> {
        self.http
            .get(url)
            .header("Authorization", &self.api_key)
            .send()
            .await?
            .json()
            .await
    }
}
