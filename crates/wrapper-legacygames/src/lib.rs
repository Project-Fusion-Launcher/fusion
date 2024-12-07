use api::models::{
    InstallerResponse, InstallerResponseData, IsExistsByEmail, Product, Products, ProductsData,
    TestLogin,
};
use base64::{prelude::BASE64_STANDARD, Engine};
use reqwest::header::{ACCEPT, AUTHORIZATION, CONTENT_LENGTH, CONTENT_TYPE};
use result::Result;
use serde::de::DeserializeOwned;
use tokio::try_join;

pub mod api;
pub mod result;
mod tests;

pub struct LegacyGamesClient {
    email: String,
    token: Option<String>,
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
        }
    }

    /// Creates a new LegacyGamesClient from an email and token.
    /// Gives access to all endpoints.
    pub fn from_token(email: String, token: String) -> Self {
        Self {
            email,
            token: Some(token),
            http: reqwest::Client::new(),
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
    pub async fn fetch_giveaway_products(&self) -> Result<Vec<Product>> {
        let response: Products = self
            .make_get_request(&api::endpoints::giveaway_catalog_by_email(&self.email))
            .await?;

        match response.data {
            ProductsData::Products(products) => Ok(products),
            ProductsData::Error(_) => Ok(Vec::new()),
        }
    }

    /// Fetches the installer for a giveaway game.
    pub async fn fetch_giveaway_installer(&self, installer_uuid: &str) -> Result<String> {
        let response: InstallerResponse = self
            .make_get_request(&api::endpoints::giveaway_installer(installer_uuid))
            .await?;

        match response.data {
            InstallerResponseData::Installer(installer) => Ok(installer.file),
            InstallerResponseData::Error(e) => Err(e.into()),
        }
    }

    /// Fetches the size of the installer for a giveaway game.
    pub async fn fetch_giveaway_installer_size(&self, installer_uuid: &str) -> Result<u32> {
        let response: InstallerResponse = self
            .make_get_request(&api::endpoints::giveaway_installer(installer_uuid))
            .await?;

        if let InstallerResponseData::Installer(installer) = response.data {
            let response = self.make_head_request(&installer.file).await?;

            if let Some(content_length) = response.headers().get(CONTENT_LENGTH) {
                Ok(content_length.to_str().unwrap().parse::<u32>()?)
            } else {
                Ok(0)
            }
        } else {
            Err("Installer not found".into())
        }
    }

    /// Fetches the purchased games. Note that a bearer token is required.
    pub async fn fetch_wp_products(&self) -> Result<Vec<Product>> {
        if self.is_email_client() {
            return Err("Token required".into());
        }

        let login = Self::test_login(self.token.clone().unwrap()).await?;

        let response: Products = self
            .make_get_request(&api::endpoints::user_downloads(login.data.user_id.unwrap()))
            .await?;

        match response.data {
            ProductsData::Products(products) => Ok(products),
            ProductsData::Error(_) => Ok(Vec::new()),
        }
    }

    /// Fetches the installer for a purchased game.
    pub async fn fetch_wp_installer(&self, product_id: u32, game_id: &str) -> Result<String> {
        if self.is_email_client() {
            return Err("Token required".into());
        }

        let response: InstallerResponse = self
            .make_get_request(&api::endpoints::wp_installer(product_id, game_id))
            .await?;

        match response.data {
            InstallerResponseData::Installer(installer) => Ok(installer.file),
            InstallerResponseData::Error(e) => Err(e.into()),
        }
    }

    /// Fetches the size of the installer for a purchased game.
    pub async fn fetch_wp_installer_size(&self, product_id: u32, game_id: &str) -> Result<u32> {
        if self.is_email_client() {
            return Err("Token required".into());
        }

        let response: InstallerResponse = self
            .make_get_request(&api::endpoints::wp_installer(product_id, game_id))
            .await?;

        if let InstallerResponseData::Installer(installer) = response.data {
            let response = self.make_head_request(&installer.file).await?;

            if let Some(content_length) = response.headers().get(CONTENT_LENGTH) {
                Ok(content_length.to_str().unwrap().parse::<u32>()?)
            } else {
                Ok(0)
            }
        } else {
            Err("Installer not found".into())
        }
    }

    /// Fetches both giveaway and wp products (if possible).
    pub async fn fetch_products(&mut self) -> Result<Vec<Product>> {
        let (mut giveaway_products, mut wp_products) =
            try_join!(self.fetch_giveaway_products(), self.fetch_wp_products())?;

        giveaway_products.append(&mut wp_products);
        Ok(giveaway_products)
    }

    /// Checks if a user exists by email.
    pub async fn fetch_user_exists(email: &str) -> Result<IsExistsByEmail> {
        Self::make_get_request_static(
            &api::endpoints::is_exsists_by_email(email),
            &reqwest::Client::new(),
            &None,
        )
        .await
    }

    /// Checks if a login is valid.
    pub async fn test_login(token: String) -> Result<TestLogin> {
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
    async fn make_get_request<D>(&self, url: &str) -> Result<D>
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
    ) -> Result<D>
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

        let response = request.send().await?.json().await?;

        Ok(response)
    }

    async fn make_head_request(&self, url: &str) -> Result<reqwest::Response> {
        let response = self.http.head(url).send().await?;

        Ok(response)
    }
}
