use api::models::{
    InstallerResponse, InstallerResponseData, IsExistsByEmail, Product, Products, ProductsData,
    TestLogin,
};
use base64::{prelude::BASE64_STANDARD, Engine};
use reqwest::{
    header::{ACCEPT, AUTHORIZATION, CONTENT_LENGTH, CONTENT_TYPE},
    RequestBuilder,
};
use serde::de::DeserializeOwned;

pub mod api;
mod tests;

pub struct LegacyGamesClient {
    email: String,
    token: Option<String>,
    user_id: Option<u32>,
    http: reqwest::Client,
}

impl LegacyGamesClient {
    /// Creates a new LegacyGamesClient from an email.
    /// Only gives access to giveaway endpoints.
    pub fn from_email(email: String) -> Self {
        Self {
            email,
            token: None,
            http: reqwest::Client::new(),
            user_id: None,
        }
    }

    /// Creates a new LegacyGamesClient from an email and token.
    /// Gives access to all endpoints.
    pub fn from_token(email: String, token: String) -> Self {
        Self {
            email,
            token: Some(token),
            http: reqwest::Client::new(),
            user_id: None,
        }
    }

    /// Checks if the client is an email client.
    pub fn is_email_client(&self) -> bool {
        self.token.is_none()
    }

    /// Checks if the client is a token client.
    pub fn is_token_client(&self) -> bool {
        self.token.is_some()
    }

    /// Fetches the giveaway games associated with the email.
    pub async fn fetch_giveaway_products(&self) -> Result<Vec<Product>, reqwest::Error> {
        let response: Products = self
            .make_get_request(&api::endpoints::get_giveaway_catalog_by_email(&self.email))
            .await?;

        match response.data {
            ProductsData::Products(products) => Ok(products),
            ProductsData::Error(_) => Ok(Vec::new()),
        }
    }

    /// Fetches the installer for a giveaway game.
    pub async fn fetch_giveaway_installer(
        &self,
        installer_uuid: &str,
    ) -> Result<RequestBuilder, &'static str> {
        let response: InstallerResponse = self
            .make_get_request(&api::endpoints::get_giveaway_installer(installer_uuid))
            .await
            .map_err(|_| "Failed to fetch installer")?;

        match response.data {
            InstallerResponseData::Installer(installer) => Ok(self
                .http
                .get(&installer.file)
                .header("Authorization", "?token?")
                .header("Accept", "application/json")
                .header("Content-Type", "application/json")),
            InstallerResponseData::Error(_) => Err("Installer not found"),
        }
    }

    /// Fetches the size of the installer for a giveaway game.
    pub async fn fetch_giveaway_installer_size(
        &self,
        installer_uuid: &str,
    ) -> Result<u32, &'static str> {
        let response: InstallerResponse = self
            .make_get_request(&api::endpoints::get_giveaway_installer(installer_uuid))
            .await
            .map_err(|_| "Failed to fetch installer")?;

        if let InstallerResponseData::Installer(installer) = response.data {
            let response = self
                .http
                .head(&installer.file)
                .send()
                .await
                .map_err(|_| "Failed to fetch installer")?;

            if let Some(content_length) = response.headers().get(CONTENT_LENGTH) {
                Ok(content_length.to_str().unwrap().parse::<u32>().unwrap())
            } else {
                Ok(0)
            }
        } else {
            Err("Installer not found")
        }
    }

    /// Fetches the installer for a wp game.
    pub async fn fetch_wp_installer(
        &self,
        product_id: u32,
        game_id: &str,
    ) -> Result<RequestBuilder, &'static str> {
        let response: InstallerResponse = self
            .make_get_request(&api::endpoints::get_wp_installer(product_id, game_id))
            .await
            .map_err(|_| "Failed to fetch installer")?;

        match response.data {
            InstallerResponseData::Installer(installer) => Ok(self
                .http
                .get(&installer.file)
                .header("Authorization", "?token?")
                .header("Accept", "application/json")
                .header("Content-Type", "application/json")),
            InstallerResponseData::Error(_) => Err("Installer not found"),
        }
    }

    /// Fetches the size of the installer for a giveaway game.
    pub async fn fetch_wp_installer_size(
        &self,
        product_id: u32,
        game_id: &str,
    ) -> Result<u32, &'static str> {
        let response: InstallerResponse = self
            .make_get_request(&api::endpoints::get_wp_installer(product_id, game_id))
            .await
            .map_err(|_| "Failed to fetch installer")?;

        if let InstallerResponseData::Installer(installer) = response.data {
            let response = self
                .http
                .head(&installer.file)
                .send()
                .await
                .map_err(|_| "Failed to fetch installer")?;

            if let Some(content_length) = response.headers().get(CONTENT_LENGTH) {
                Ok(content_length.to_str().unwrap().parse::<u32>().unwrap())
            } else {
                Ok(0)
            }
        } else {
            Err("Installer not found")
        }
    }

    /// Fetches the purchased games. Note that a bearer token is required.
    pub async fn fetch_products(&mut self) -> Result<Vec<Product>, reqwest::Error> {
        if self.user_id.is_none() {
            let login = Self::test_login(self.token.clone().unwrap()).await.unwrap();
            self.user_id = login.data.user_id;
        }

        let response: Products = self
            .make_get_request(&api::endpoints::get_user_downloads(self.user_id.unwrap()))
            .await?;

        match response.data {
            ProductsData::Products(products) => Ok(products),
            ProductsData::Error(_) => Ok(Vec::new()),
        }
    }

    /// Checks if a user exists by email.
    pub async fn fetch_user_exists(email: &str) -> Result<IsExistsByEmail, reqwest::Error> {
        Self::make_get_request_static(
            &api::endpoints::is_exsists_by_email(email),
            &reqwest::Client::new(),
            &None,
        )
        .await
    }

    /// Checks if a login is valid.
    pub async fn test_login(token: String) -> Result<TestLogin, reqwest::Error> {
        Self::make_get_request_static(
            &api::endpoints::login(),
            &reqwest::Client::new(),
            &Some(token),
        )
        .await
    }

    /// Generates a token for the given username and password.
    pub fn generate_token(username: &str, password: &str) -> String {
        let merged_string = format!("{}:{}", username, password);
        BASE64_STANDARD.encode(merged_string.as_bytes())
    }

    /// Makes a GET request to the Legacy Games API.
    /// This function is not static and requires a LegacyGamesClient to be created,
    /// in order to avoid creating multiple reqwest::Client instances.
    async fn make_get_request<D>(&self, url: &str) -> Result<D, reqwest::Error>
    where
        D: DeserializeOwned,
    {
        Self::make_get_request_static(url, &self.http, &self.token).await
    }

    /// Makes a GET request to the Legacy Games API.
    /// This function is static and can be used without creating a LegacyGamesClient.
    async fn make_get_request_static<D>(
        url: &str,
        http: &reqwest::Client,
        token: &Option<String>,
    ) -> Result<D, reqwest::Error>
    where
        D: DeserializeOwned,
    {
        let mut request = http
            .get(url)
            .header(AUTHORIZATION, "?token?")
            .header(ACCEPT, "application/json")
            .header(CONTENT_TYPE, "application/json");

        if token.is_some() {
            request = request.header("UserToken", format!("Basic {}", token.clone().unwrap()));
        }

        request.send().await?.json().await
    }
}
