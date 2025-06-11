use super::{endpoints, models::*};
use anyhow::{Context, Result, anyhow};
use reqwest::{
    IntoUrl, Url,
    header::{AUTHORIZATION, CONTENT_TYPE, USER_AGENT},
};
use serde::de::DeserializeOwned;
use sha1::{Digest, Sha1};
use tokio::{
    fs::OpenOptions,
    io::{AsyncReadExt, AsyncWriteExt},
    sync::oneshot,
};
use tokio_util::bytes::Bytes;

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
            AssetsResponse::Error(e) => Err(anyhow!(e.error_code)),
        }
    }

    pub async fn fetch_game_info(&self, namespace: &str, catalog_item_id: &str) -> Result<Game> {
        let url = endpoints::game_info(namespace, catalog_item_id);
        let response: GameInfoResponse = self.get_json(url).await?;
        match response {
            GameInfoResponse::Game(game) => Ok(*game),
            GameInfoResponse::Error(e) => Err(anyhow!(e.error_code)),
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

    /*pub async fn list_stored_manifests(&self, game_id: &str) -> Result<Vec<String>> {
        Self::list_manifests(game_id).await
    }

    pub async fn get_game_manifest(
        &self,
        platform: &str,
        catalog_item_id: &str,
        version: &str,
    ) -> Result<Manifest> {
        if let Ok(bytes) = Self::read_manifest(catalog_item_id, version).await {
            if bytes.first() == Some(&b'{') {
                if let Ok(json_manifest) = serde_json::from_slice::<JsonManifest>(&bytes) {
                    return Ok(Manifest::from(json_manifest));
                }
            }

            Manifest::new(bytes)
        } else {
            let assets = self.fetch_game_assets(platform).await?;
            let asset = assets
                .into_iter()
                .find(|asset| asset.catalog_item_id == catalog_item_id)
                .context("Game not found")?;

            self.fetch_game_manifest(
                platform,
                &asset.namespace,
                catalog_item_id,
                &asset.app_name,
                version,
            )
            .await
        }
    }

    async fn fetch_game_manifest(
        &self,
        platform: &str,
        namespace: &str,
        catalog_item_id: &str,
        app_name: &str,
        version: &str,
    ) -> Result<Manifest> {
        let element = self
            .fetch_game_manifest_element(platform, namespace, catalog_item_id, app_name)
            .await?;

        if element.build_version != version {
            return Err(anyhow!("Version mismatch"));
        }

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

            let catalog_item_id = catalog_item_id.to_string();
            let version = version.to_string();
            rayon::spawn(move || {
                let result = (|| {
                    let computed_sha1 = format!("{:x}", Sha1::digest(&bytes));
                    if computed_sha1 != hash {
                        return Err("SHA1 hash mismatch".into());
                    }

                    let _ = block_on(async {
                        Self::write_manifest(&bytes, &catalog_item_id, &version).await
                    });

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
    }*/

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
            GameManifestResponse::Elements(elements) => {
                elements.into_iter().next().context("No elements found")
            }
            GameManifestResponse::Error(e) => Err(anyhow!(e.error_code)),
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
            AccessResponse::Error(e) => Err(anyhow!(e.error_code)),
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

    /*async fn list_manifests(game_id: &str) -> Result<Vec<String>> {
        let app_data = APP.get().unwrap().path().app_data_dir().unwrap();
        let mut manifest_path = app_data.to_path_buf();
        manifest_path.push("storefronts");
        manifest_path.push("epicgames");
        manifest_path.push("manifests");
        manifest_path.push(game_id);

        if !manifest_path.exists() {
            return Ok(Vec::new());
        }

        let mut entries = tokio::fs::read_dir(&manifest_path).await?;
        let mut manifests = Vec::new();

        while let Some(entry) = entries.next_entry().await? {
            if entry.file_type().await?.is_file() {
                if let Some(name) = entry.file_name().to_str() {
                    manifests.push(name.to_string());
                }
            }
        }

        Ok(manifests)
    }

    async fn write_manifest(bytes: &Bytes, game_id: &str, version: &str) -> Result<()> {
        let app_data = APP.get().unwrap().path().app_data_dir().unwrap();
        let mut manifest_path = app_data.to_path_buf();
        manifest_path.push("storefronts");
        manifest_path.push("epicgames");
        manifest_path.push("manifests");
        manifest_path.push(game_id);
        manifest_path.push(version);

        let mut file = OpenOptions::new()
            .create(true)
            .write(true)
            .truncate(true)
            .open_with_dirs(&manifest_path)
            .await?;

        file.write_all(bytes).await?;

        Ok(())
    }

    async fn read_manifest(game_id: &str, version: &str) -> Result<Vec<u8>> {
        let app_data = APP.get().unwrap().path().app_data_dir().unwrap();
        let mut manifest_path = app_data.to_path_buf();
        manifest_path.push("storefronts");
        manifest_path.push("epicgames");
        manifest_path.push("manifests");
        manifest_path.push(game_id);
        manifest_path.push(version);

        let mut file = OpenOptions::new().read(true).open(&manifest_path).await?;

        let mut manifest = Vec::new();
        file.read_to_end(&mut manifest).await?;

        Ok(manifest)
    }*/
}
