use api::{
    endpoints,
    models::{
        chunk::Chunk, manifest::Manifest, AccessTokenResponse, Asset, CategoryPath, Game,
        GameInfoResponse, GameManifestsResponse, GrantType, LoginParams,
    },
};
use futures::{stream::FuturesUnordered, StreamExt};
use reqwest::{
    header::{AUTHORIZATION, CONTENT_TYPE, USER_AGENT},
    StatusCode, Url,
};
use serde::de::DeserializeOwned;
use sha1::{Digest, Sha1};
use std::sync::Arc;
use tokio::sync::Semaphore;

pub mod api;
mod tests;

pub struct EpicGamesClient {
    access_token: String,
    refresh_token: String,
    http: Arc<reqwest::Client>,
}

impl EpicGamesClient {
    pub async fn from_code<S: AsRef<str>>(code: S) -> Result<Self, reqwest::Error> {
        Self::authenticate(
            GrantType::AuthorizationCode,
            Some(code.as_ref().to_string()),
            None,
        )
        .await
    }

    pub async fn from_refresh_token<S: AsRef<str>>(
        refresh_token: S,
    ) -> Result<Self, reqwest::Error> {
        Self::authenticate(
            GrantType::RefreshToken,
            None,
            Some(refresh_token.as_ref().to_string()),
        )
        .await
    }

    pub async fn from_access_token<S: AsRef<str>>(access_token: S) -> Result<Self, reqwest::Error> {
        let http = reqwest::Client::new();
        Ok(Self {
            access_token: access_token.as_ref().to_string(),
            refresh_token: String::new(),
            http: Arc::new(http),
        })
    }

    pub fn refresh_token(&self) -> String {
        self.refresh_token.clone()
    }

    pub async fn fetch_games(&self) -> Result<Vec<Game>, reqwest::Error> {
        let assets: Vec<Asset> =
            Self::make_get_request(&self.http, api::endpoints::assets(), &self.access_token)
                .await?;
        let semaphore = Arc::new(Semaphore::new(16));
        let mut tasks = FuturesUnordered::new();

        for asset in assets {
            if asset.namespace == "ue" {
                continue;
            }

            let semaphore = Arc::clone(&semaphore);
            let http = Arc::clone(&self.http);
            let access_token = self.access_token.clone();

            tasks.push(tokio::spawn(async move {
                let _permit = semaphore.clone().acquire_owned().await.unwrap();

                let result: Result<GameInfoResponse, reqwest::Error> = Self::make_get_request(
                    &http,
                    &endpoints::game_info(&asset.namespace, &asset.catalog_item_id),
                    &access_token,
                )
                .await;

                result.map(|game_info| game_info.game)
            }));
        }

        let mut games = Vec::new();
        while let Some(result) = tasks.next().await {
            match result {
                Ok(Ok(game)) => {
                    if game.main_game_item.is_none()
                        && game
                            .categories
                            .iter()
                            .any(|c| c.path == CategoryPath::Games)
                    {
                        games.push(game)
                    }
                }
                Ok(Err(e)) => eprintln!("Error fetching game info: {:?}", e),
                Err(e) => eprintln!("Task error: {:?}", e),
            }
        }

        Ok(games)
    }

    pub async fn fetch_game_versions(
        &self,
        catalog_item_id: &str,
    ) -> Result<Vec<String>, &'static str> {
        let assets: Vec<Asset> =
            Self::make_get_request(&self.http, api::endpoints::assets(), &self.access_token)
                .await
                .map_err(|_| "Failed to fetch assets")?;

        let asset = assets
            .into_iter()
            .find(|asset| asset.catalog_item_id == catalog_item_id)
            .ok_or("Game not found in assets")?;

        let response: GameManifestsResponse = Self::make_get_request(
            &self.http,
            &api::endpoints::game_manifests(&asset.namespace, catalog_item_id, &asset.app_name),
            &self.access_token,
        )
        .await
        .map_err(|_| "Failed to fetch game info")?;

        Ok(response
            .elements
            .into_iter()
            .map(|e| e.build_version)
            .collect())
    }

    pub async fn fetch_version_manifest(
        &self,
        catalog_item_id: &str,
        build_version: &str,
    ) -> Result<Manifest, &'static str> {
        let assets: Vec<Asset> =
            Self::make_get_request(&self.http, api::endpoints::assets(), &self.access_token)
                .await
                .map_err(|_| "Failed to fetch assets")?;

        let asset = assets
            .into_iter()
            .find(|asset| asset.catalog_item_id == catalog_item_id)
            .ok_or("Game not found in assets")?;

        let response: GameManifestsResponse = Self::make_get_request(
            &self.http,
            &api::endpoints::game_manifests(&asset.namespace, catalog_item_id, &asset.app_name),
            &self.access_token,
        )
        .await
        .map_err(|_| "Failed to fetch game info")?;

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

            let response = self
                .http
                .get(url)
                .header(AUTHORIZATION, format!("bearer {}", self.access_token))
                .header(
                    USER_AGENT,
                    "UELauncher/11.0.1-14907503+++Portal+Release-Live Windows/10.0.19041.1.256.64bit",
                )
                .send()
                .await
                .map_err(|_| "Failed to fetch manifest")?;

            if StatusCode::is_success(&response.status()) {
                let bytes = response
                    .bytes()
                    .await
                    .map_err(|_| "Failed to read manifest response")?;

                let mut hasher = Sha1::new();
                hasher.update(&bytes);
                let hash = hasher.finalize();

                if format!("{:x}", hash) != element.hash {
                    continue;
                }

                return Manifest::from_bytes(&bytes);
            }
        }

        Err("Failed to fetch manifest")
    }

    pub async fn fetch_cdn_urls(
        &self,
        catalog_item_id: &str,
        build_version: &str,
    ) -> Result<Vec<Url>, &'static str> {
        let assets: Vec<Asset> =
            Self::make_get_request(&self.http, api::endpoints::assets(), &self.access_token)
                .await
                .map_err(|_| "Failed to fetch assets")?;

        let asset = assets
            .into_iter()
            .find(|asset| asset.catalog_item_id == catalog_item_id)
            .ok_or("Game not found in assets")?;

        let response: GameManifestsResponse = Self::make_get_request(
            &self.http,
            &api::endpoints::game_manifests(&asset.namespace, catalog_item_id, &asset.app_name),
            &self.access_token,
        )
        .await
        .map_err(|_| "Failed to fetch game info")?;

        let element = response
            .elements
            .into_iter()
            .find(|e| e.build_version == build_version)
            .ok_or("Version not found")?;

        let mut result = Vec::new();

        for manifest_url in element.manifests {
            let mut url = Url::parse(&manifest_url.uri).map_err(|_| "Invalid URL")?;

            {
                let mut segments = url.path_segments_mut().map_err(|_| "Cannot be base URL")?;
                segments.pop();
            }

            result.push(url);
        }

        Ok(result)
    }

    async fn make_get_request<D>(
        http: &reqwest::Client,
        url: &str,
        access_token: &str,
    ) -> Result<D, reqwest::Error>
    where
        D: DeserializeOwned,
    {
        http.get(url)
            .header(AUTHORIZATION, format!("bearer {}", access_token))
            .header(
                USER_AGENT,
                "UELauncher/11.0.1-14907503+++Portal+Release-Live Windows/10.0.19041.1.256.64bit",
            )
            .send()
            .await?
            .json()
            .await
    }

    async fn authenticate(
        grant_type: GrantType,
        code: Option<String>,
        refresh_token: Option<String>,
    ) -> Result<Self, reqwest::Error> {
        let params = LoginParams {
            grant_type,
            token_type: String::from("eg1"),
            code,
            refresh_token,
        };

        let http = Arc::new(reqwest::Client::new());

        let response: AccessTokenResponse = http
            .post(api::endpoints::access_token())
            .header(CONTENT_TYPE, "application/x-www-form-urlencoded")
            .header(
                USER_AGENT,
                "UELauncher/11.0.1-14907503+++Portal+Release-Live Windows/10.0.19041.1.256.64bit",
            )
            .basic_auth(
                "34a02cf8f4414e29b15921876da36f9a",
                Some("daafbccc737745039dffe53d94fc76cf"),
            )
            .body(serde_urlencoded::to_string(&params).unwrap())
            .send()
            .await?
            .json()
            .await?;

        Ok(Self {
            access_token: response.access_token,
            refresh_token: response.refresh_token,
            http,
        })
    }

    pub fn decode_chunk(data: &[u8]) -> Result<Chunk, &'static str> {
        Chunk::from_bytes(data)
    }
}
