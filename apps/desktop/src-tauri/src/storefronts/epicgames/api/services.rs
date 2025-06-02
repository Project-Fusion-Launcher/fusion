use super::{endpoints, models::*};
use crate::common::result::Result;
use reqwest::{
    header::{AUTHORIZATION, CONTENT_TYPE, USER_AGENT},
    IntoUrl, Url,
};
use serde::de::DeserializeOwned;
use sha1::{Digest, Sha1};
use tokio::sync::oneshot;

pub struct Services {
    http: reqwest::Client,
    access_token: String,
    refresh_token: String,
}

impl Services {
    pub fn refresh_token(&self) -> String {
        self.refresh_token.clone()
    }

    pub async fn from_refresh_token(token: String) -> Result<Self> {
        let http = reqwest::Client::new();

        let response =
            Self::authenticate(&http, GrantType::RefreshToken, None, Some(token)).await?;

        println!("[Epic Games] Logged in as: {}", response.display_name);

        Ok(Self {
            http,
            access_token: response.access_token,
            refresh_token: response.refresh_token,
        })
    }

    pub async fn fetch_game_assets(&self, platform: &str) -> Result<Vec<Asset>> {
        let url = endpoints::assets(platform, "Live");
        let response: AssetsResponse = self.get_json(url).await?;
        match response {
            AssetsResponse::Success(assets) => Ok(assets),
            AssetsResponse::Error(e) => Err(e.error_code.into()),
        }
    }

    pub async fn fetch_game_info(&self, namespace: &str, catalog_item_id: &str) -> Result<Game> {
        let url = endpoints::game_info(namespace, catalog_item_id);
        let response: GameInfoResponse = self.get_json(url).await?;
        match response {
            GameInfoResponse::Game(game) => Ok(*game),
            GameInfoResponse::Error(e) => Err(e.error_code.into()),
        }
    }

    pub async fn fetch_cdn_urls(
        &self,
        platform: &str,
        namespace: &str,
        catalog_item_id: &str,
        app_name: &str,
    ) -> Result<Vec<Url>> {
        let element = self
            .fetch_game_manifest_element(platform, namespace, catalog_item_id, app_name)
            .await?;

        let urls = element
            .manifests
            .iter()
            .filter_map(|manifest| Url::parse(&manifest.uri).ok())
            .collect();

        Ok(urls)
    }

    pub async fn fetch_game_manifest(
        &self,
        platform: &str,
        namespace: &str,
        catalog_item_id: &str,
        app_name: &str,
    ) -> Result<Manifest> {
        let element = self
            .fetch_game_manifest_element(platform, namespace, catalog_item_id, app_name)
            .await?;

        for manifest_url in element.manifests.iter() {
            let mut url = match Url::parse(&manifest_url.uri) {
                Ok(u) => u,
                Err(_) => continue,
            };

            for param in &manifest_url.query_params {
                url.query_pairs_mut().append_pair(&param.name, &param.value);
            }

            let response = match self.get(url).await {
                Ok(resp) if resp.status().is_success() => resp,
                _ => continue,
            };

            let bytes = match response.bytes().await {
                Ok(b) => b,
                Err(_) => continue,
            };

            let (tx, rx) = oneshot::channel();
            let hash = element.hash.clone();

            rayon::spawn(move || {
                let result = (|| {
                    let computed_sha1 = format!("{:x}", Sha1::digest(&bytes));
                    if computed_sha1 != hash {
                        return Err("SHA1 hash mismatch".into());
                    }

                    if bytes.first() == Some(&b'{') {
                        if let Ok(json_manifest) = serde_json::from_slice::<JsonManifest>(&bytes) {
                            return Ok(Manifest::from(json_manifest));
                        }
                    }

                    Manifest::new(bytes.into())
                })();

                let _ = tx.send(result);
            });

            match rx.await {
                Ok(manifest) => return manifest,
                Err(_) => continue,
            }
        }

        Err("Failed to fetch manifest".into())
    }

    async fn fetch_game_manifest_element(
        &self,
        platform: &str,
        namespace: &str,
        catalog_item_id: &str,
        app_name: &str,
    ) -> Result<GameManifestElement> {
        let url = endpoints::game_manifest(platform, namespace, catalog_item_id, app_name, "Live");
        let response: GameManifestResponse = self.get_json(url).await?;

        match response {
            GameManifestResponse::Elements(elements) => elements
                .into_iter()
                .next()
                .ok_or_else(|| "No elements found".into()),
            GameManifestResponse::Error(e) => Err(e.error_code.into()),
        }
    }

    async fn authenticate(
        http: &reqwest::Client,
        grant_type: GrantType,
        code: Option<String>,
        refresh_token: Option<String>,
    ) -> Result<AccessData> {
        let params = LoginParams {
            grant_type,
            token_type: "eg1",
            code,
            refresh_token,
        };

        let response: AccessResponse = http
            .post(endpoints::oauth_token())
            .header(CONTENT_TYPE, "application/x-www-form-urlencoded")
            .header(USER_AGENT, super::USER_AGENT)
            .basic_auth(super::USER_BASIC, Some(super::PASSWORD_BASIC))
            .body(serde_urlencoded::to_string(&params)?)
            .send()
            .await?
            .json()
            .await?;

        match response {
            AccessResponse::Success(data) => Ok(data),
            AccessResponse::Error(e) => Err(e.error_code.into()),
        }
    }

    async fn get<U>(&self, url: U) -> Result<reqwest::Response>
    where
        U: IntoUrl,
    {
        Ok(self
            .http
            .get(url)
            .header(AUTHORIZATION, format!("bearer {}", self.access_token))
            .header(USER_AGENT, super::USER_AGENT)
            .send()
            .await?)
    }

    async fn get_json<D, U>(&self, url: U) -> Result<D>
    where
        D: DeserializeOwned,
        U: IntoUrl,
    {
        Ok(self.get(url).await?.json().await?)
    }
}
