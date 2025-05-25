use super::{endpoints, models::*};
use crate::common::result::Result;
use reqwest::{
    header::{ACCEPT, AUTHORIZATION, CONTENT_LENGTH, CONTENT_TYPE},
    IntoUrl,
};
use serde::de::DeserializeOwned;
use tokio::try_join;

pub struct Services {
    http: reqwest::Client,
    email: String,
    token: Option<String>,
    user_id: Option<u32>,
}

impl Services {
    pub async fn from_email(email: String) -> Result<Self> {
        let services = Self {
            http: reqwest::Client::new(),
            email,
            token: None,
            user_id: None,
        };

        if let Ok(response) = services.fetch_user_exists().await {
            if response.status.is_success() {
                return Ok(services);
            }
        }

        Err("User does not exist".into())
    }

    pub async fn from_token(email: String, token: String) -> Result<Self> {
        let mut services = Self::from_email(email).await?;
        services.token = Some(token);
        if let Ok(response) = services.fetch_test_login().await {
            if let Some(user_id) = response.data.user_id {
                services.user_id = Some(user_id);
                return Ok(services);
            }
        }

        Err("Invalid user token".into())
    }

    pub async fn fetch_products(&self) -> Result<Vec<Product>> {
        let (mut giveaway_products, mut wp_products) = if self.is_wp() {
            try_join!(self.fetch_giveaway_products(), self.fetch_wp_products())?
        } else {
            (self.fetch_giveaway_products().await?, Vec::new())
        };

        giveaway_products.append(&mut wp_products);
        Ok(giveaway_products)
    }

    pub async fn fetch_giveaway_installer(&self, installer_uuid: &str) -> Result<String> {
        let url = endpoints::giveaway_installer(installer_uuid);
        let response: InstallerResponse = self.get(url).await?;
        match response.data {
            InstallerResponseData::Installer(installer) => Ok(installer.file),
            InstallerResponseData::Error(e) => Err(e.into()),
        }
    }

    pub async fn fetch_giveaway_installer_size(&self, installer_uuid: &str) -> Result<u32> {
        let url = self.fetch_giveaway_installer(installer_uuid).await?;
        let response = self.head(url).await?;

        if let Some(content_length) = response.headers().get(CONTENT_LENGTH) {
            Ok(content_length.to_str().unwrap().parse()?)
        } else {
            Ok(0)
        }
    }

    pub async fn fetch_wp_installer(&self, product_id: u32, game_id: &str) -> Result<String> {
        if !self.is_wp() {
            return Err("Token required".into());
        }

        let url = endpoints::wp_installer(product_id, game_id);
        let response: InstallerResponse = self.get(url).await?;
        match response.data {
            InstallerResponseData::Installer(installer) => Ok(installer.file),
            InstallerResponseData::Error(e) => Err(e.into()),
        }
    }

    pub async fn fetch_wp_installer_size(&self, product_id: u32, game_id: &str) -> Result<u32> {
        if !self.is_wp() {
            return Err("Token required".into());
        }

        let url = self.fetch_wp_installer(product_id, game_id).await?;
        let response = self.head(url).await?;

        if let Some(content_length) = response.headers().get(CONTENT_LENGTH) {
            Ok(content_length.to_str().unwrap().parse()?)
        } else {
            Ok(0)
        }
    }

    async fn fetch_giveaway_products(&self) -> Result<Vec<Product>> {
        let url = endpoints::giveaway_catalog_by_email(&self.email);
        let products: Products = self.get(url).await?;

        match products.data {
            ProductsData::Products(products) => Ok(products
                .into_iter()
                .map(|mut product| {
                    product.is_giveaway = true;
                    product
                })
                .collect()),
            ProductsData::Error(e) => Err(e.into()),
        }
    }

    async fn fetch_wp_products(&self) -> Result<Vec<Product>> {
        if !self.is_wp() {
            return Err("Token required".into());
        }

        let url = endpoints::user_downloads(self.user_id.unwrap());
        let products: Products = self.get(url).await?;

        match products.data {
            ProductsData::Products(products) => Ok(products),
            ProductsData::Error(_) => Ok(Vec::new()),
        }
    }

    pub fn is_wp(&self) -> bool {
        self.token.is_some()
    }

    async fn fetch_user_exists(&self) -> Result<IsExistsByEmail> {
        let url = endpoints::user_exists_by_email(&self.email);
        self.get(url).await
    }

    async fn fetch_test_login(&self) -> Result<TestLogin> {
        let url = endpoints::user_login();
        self.get(url).await
    }

    async fn get<D, U>(&self, url: U) -> Result<D>
    where
        D: DeserializeOwned,
        U: IntoUrl,
    {
        let mut request = self
            .http
            .get(url)
            .header(AUTHORIZATION, "?token?")
            .header(ACCEPT, "application/json")
            .header(CONTENT_TYPE, "application/json");

        if let Some(token) = &self.token {
            request = request.header("UserToken", format!("Basic {}", token));
        }

        Ok(request.send().await?.json().await?)
    }

    async fn head<U: IntoUrl>(&self, url: U) -> Result<reqwest::Response> {
        Ok(self.http.head(url).send().await?)
    }
}
