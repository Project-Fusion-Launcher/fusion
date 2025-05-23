use super::{endpoints, models::*};
use crate::common::result::Result;
use reqwest::{
    header::{AUTHORIZATION, CONTENT_TYPE, USER_AGENT},
    IntoUrl, Url,
};
use serde::de::DeserializeOwned;

pub struct Services {
    http: reqwest::Client,
    access_token: String,
    refresh_token: String,
}

impl Services {
    pub fn refresh_token(&self) -> String {
        self.refresh_token.clone()
    }

    pub async fn from_refresh_token(refresh_token: String) -> Result<Self> {
        let http = reqwest::Client::new();

        let response =
            Self::authenticate(&http, GrantType::RefreshToken, None, Some(refresh_token)).await?;

        Ok(Self {
            http,
            access_token: response.access_token,
            refresh_token: response.refresh_token,
        })
    }

    pub async fn fetch_game_assets(&self, platform: &str) -> Result<Vec<Asset>> {
        let url = endpoints::assets(platform, "Live");
        self.get(url).await
    }

    pub async fn fetch_game_info(&self, namespace: &str, catalog_item_id: &str) -> Result<Game> {
        let url = endpoints::game_info(namespace, catalog_item_id);
        let response: GameInfoResponse = self.get(url).await?;
        Ok(response.game)
    }

    pub async fn fetch_game_manifest(
        &self,
        platform: &str,
        namespace: &str,
        catalog_item_id: &str,
        app_name: &str,
        build_version: &str,
    ) -> Result<()> {
        let url = endpoints::game_manifest(platform, namespace, catalog_item_id, app_name, "Live");
        let response: GameManifestResponse = self.get(url).await?;

        let element = response
            .elements
            .into_iter()
            .find(|e| e.build_version == build_version)
            .ok_or("Version not found")?;

        for manifest_url in element.manifests {
            let mut url = Url::parse(&manifest_url.uri).map_err(|_| "Invalid URL")?;

            for param in &manifest_url.query_params {
                url.query_pairs_mut().append_pair(&param.name, &param.value);
            }
        }

        Ok(())
    }

    async fn authenticate(
        http: &reqwest::Client,
        grant_type: GrantType,
        code: Option<String>,
        refresh_token: Option<String>,
    ) -> Result<AccessTokenResponse> {
        let params = LoginParams {
            grant_type,
            token_type: String::from("eg1"),
            code,
            refresh_token,
        };

        Ok(http
            .post(endpoints::oauth_token())
            .header(CONTENT_TYPE, "application/x-www-form-urlencoded")
            .header(USER_AGENT, super::USER_AGENT)
            .basic_auth(super::USER_BASIC, Some(super::PASSWORD_BASIC))
            .body(serde_urlencoded::to_string(&params).unwrap())
            .send()
            .await?
            .json()
            .await?)
    }

    async fn get<D, U>(&self, url: U) -> Result<D>
    where
        D: DeserializeOwned,
        U: IntoUrl,
    {
        Ok(self
            .http
            .get(url)
            .header(AUTHORIZATION, format!("bearer {}", self.access_token))
            .header(USER_AGENT, super::USER_AGENT)
            .send()
            .await?
            .json()
            .await?)
    }
}
