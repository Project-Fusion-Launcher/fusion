use crate::api::services::Services;
use anyhow::Result;
use async_trait::async_trait;
use database::models::Config;
use std::sync::{Arc, OnceLock};
use storefront::StorefrontClient;
use tokio::sync::RwLock;

mod api;

static LEGACY_GAMES: OnceLock<Arc<RwLock<LegacyGamesClient>>> = OnceLock::new();

#[derive(Default)]
pub struct LegacyGamesClient {
    services: Option<Services>,
}

impl LegacyGamesClient {
    pub fn get_client() -> Arc<RwLock<LegacyGamesClient>> {
        LEGACY_GAMES
            .get_or_init(|| Arc::new(RwLock::new(LegacyGamesClient::default())))
            .clone()
    }
}

#[async_trait]
impl StorefrontClient for LegacyGamesClient {
    async fn init(&mut self, config: Config) -> Result<()> {
        let email = config.lg_email();
        let token = config.lg_token();

        if email.is_none() {
            return Ok(());
        }

        let services = match token {
            Some(token) => Services::from_token(email.unwrap(), token).await?,
            None => Services::from_email(email.unwrap()).await?,
        };

        self.services = Some(services);

        Ok(())
    }
}
