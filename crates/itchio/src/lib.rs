use crate::api::{models::GameClassification, services::Services};
use anyhow::Result;
use async_trait::async_trait;
use database::models::{Config, Game};
use std::sync::{Arc, OnceLock};
use storefront::StorefrontClient;
use tokio::sync::RwLock;

mod api;
mod conversions;

static ITCHIO: OnceLock<Arc<RwLock<ItchioClient>>> = OnceLock::new();

#[derive(Default)]
pub struct ItchioClient {
    services: Option<Arc<Services>>,
}

impl ItchioClient {
    pub fn get_client() -> Arc<RwLock<ItchioClient>> {
        ITCHIO
            .get_or_init(|| Arc::new(RwLock::new(ItchioClient::default())))
            .clone()
    }
}

#[async_trait]
impl StorefrontClient for ItchioClient {
    async fn init(&mut self, config: Config) -> Result<()> {
        let api_key = config.it_api_key();

        let api_key = match api_key {
            Some(key) => key,
            None => return Ok(()),
        };

        self.services = Some(Arc::new(Services::from_api_key(api_key).await?));

        Ok(())
    }

    async fn fetch_games(&self) -> Result<Vec<Game>> {
        let services = match &self.services {
            Some(s) => s,
            None => return Ok(vec![]),
        };

        let mut games = Vec::new();
        let mut page = 1;

        loop {
            let owned_keys = services.fetch_owned_keys(page).await?;
            let current_page_count = owned_keys.owned_keys.len() as u8;

            games.extend(
                owned_keys
                    .owned_keys
                    .into_iter()
                    .filter(|key| key.game.classification == GameClassification::Game)
                    .map(Game::from),
            );

            if current_page_count < owned_keys.per_page {
                break;
            }

            page += 1;
        }

        Ok(games)
    }
}
