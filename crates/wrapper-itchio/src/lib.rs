mod api;
mod tests;

use api::models::{
    Build, BuildResponse, Builds, Collection, CollectionGames, CollectionsResponse, Login,
    LoginParams, OwnedKeys, ScannedArchive, ScannedArchiveResponse, TOTPLoginParams, Upload,
    UploadResponse, Uploads,
};
use reqwest::header::CONTENT_TYPE;
use serde::de::DeserializeOwned;
use serde::Serialize;

pub struct ItchioClient {
    api_key: String,
    http: reqwest::Client,
}

impl ItchioClient {
    /// Creates a new ItchioClient associated with the given API key.
    pub fn new<S: AsRef<str>>(api_key: S) -> Self {
        Self {
            api_key: api_key.as_ref().to_string(),
            http: reqwest::Client::new(),
        }
    }

    /// Fetches the list of keys (games) that the user owns.
    pub async fn fetch_owned_keys(&self, page: u32) -> Result<OwnedKeys, reqwest::Error> {
        self.make_get_request(&api::endpoints::owned_keys(page))
            .await
    }

    /// Fetches the list of uploads for a game.
    pub async fn fetch_game_uploads(
        &self,
        game_id: u32,
        download_key_id: u32,
    ) -> Result<Uploads, reqwest::Error> {
        self.make_get_request(&api::endpoints::uploads(game_id, download_key_id))
            .await
    }

    /// Fetches a specific upload.
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

    /// Fetches information about the scanned upload.
    pub async fn fetch_upload_scanned_archive(
        &self,
        upload_id: u32,
        download_key_id: u32,
    ) -> Result<ScannedArchive, reqwest::Error> {
        let response: ScannedArchiveResponse = self
            .make_get_request(&api::endpoints::upload_scanned_archive(
                upload_id,
                download_key_id,
            ))
            .await?;
        Ok(response.scanned_archive)
    }

    /// Fetches the list of builds associated to an upload.
    pub async fn fetch_upload_builds(
        &self,
        upload_id: u32,
        download_key_id: u32,
    ) -> Result<Builds, reqwest::Error> {
        self.make_get_request(&api::endpoints::builds(upload_id, download_key_id))
            .await
    }

    /// Fetches a specific build.
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

    /// Fetches information about the scanned build.
    pub async fn fetch_build_scanned_archive(
        &self,
        build_id: u32,
        download_key_id: u32,
    ) -> Result<ScannedArchive, reqwest::Error> {
        let response: ScannedArchiveResponse = self
            .make_get_request(&api::endpoints::build_scanned_archive(
                build_id,
                download_key_id,
            ))
            .await?;
        Ok(response.scanned_archive)
    }

    /// Fetches the list of collections that the user has created.
    pub async fn fetch_collections(&self) -> Result<Vec<Collection>, reqwest::Error> {
        let response: CollectionsResponse = self
            .make_get_request(&api::endpoints::collections())
            .await?;
        Ok(response.collections)
    }

    /// Fetches the list of games in a collection.
    pub async fn fetch_collection_games(
        &self,
        collection_id: u32,
        page: u32,
    ) -> Result<CollectionGames, reqwest::Error> {
        self.make_get_request(&api::endpoints::collection_games(collection_id, page))
            .await
    }

    /// Static function to login to the itch.io API.
    pub async fn login(username: String, password: String) -> Result<Login, reqwest::Error> {
        Self::login_with_recaptcha(username, password, String::from("")).await
    }

    /// Static function to log in to the itch.io API using reCAPTCHA.
    /// The recaptcha parameter is the response token from the reCAPTCHA widget,
    /// whose URL is obtained previously from the [login](ItchioClient::login) response.
    pub async fn login_with_recaptcha(
        username: String,
        password: String,
        recaptcha: String,
    ) -> Result<Login, reqwest::Error> {
        let params = LoginParams {
            source: "desktop",
            username,
            password,
            recaptcha_response: if recaptcha.is_empty() {
                None
            } else {
                Some(recaptcha)
            },
        };

        let response: Login = Self::make_post_request(&api::endpoints::login(), params).await?;

        Ok(response)
    }

    /// Static function to login to the itch.io API using TOTP.
    pub async fn totp_verify(code: String, token: String) -> Result<Login, reqwest::Error> {
        let params = TOTPLoginParams { token, code };

        let response: Login =
            Self::make_post_request(&api::endpoints::totp_verify(), params).await?;

        Ok(response)
    }

    /// Makes a GET request to the itch.io API.
    async fn make_get_request<D>(&self, url: &str) -> Result<D, reqwest::Error>
    where
        D: DeserializeOwned,
    {
        self.http
            .get(url)
            .header("Authorization", &self.api_key)
            .send()
            .await?
            .json()
            .await
    }

    /// Makes a POST request to the itch.io API.
    /// This function does not depend on self, so it can be called statically
    /// even without an API key.
    async fn make_post_request<D, S>(url: &str, params: S) -> Result<D, reqwest::Error>
    where
        D: DeserializeOwned,
        S: Serialize,
    {
        reqwest::Client::new()
            .post(url)
            .header(CONTENT_TYPE, "application/x-www-form-urlencoded")
            .body(serde_urlencoded::to_string(&params).unwrap())
            .send()
            .await?
            .json()
            .await
    }
}
