use crate::api::services::Services;
use anyhow::Result;
use async_trait::async_trait;
use database::models::Config;
use std::sync::{Arc, OnceLock};
use storefront::StorefrontClient;
use tokio::sync::RwLock;

mod api;

static EPIC_GAMES: OnceLock<Arc<RwLock<EpicGamesClient>>> = OnceLock::new();

#[derive(Default)]
pub struct EpicGamesClient {
    services: Option<Arc<Services>>,
}

impl EpicGamesClient {
    pub fn get_epic_games() -> Arc<RwLock<EpicGamesClient>> {
        EPIC_GAMES
            .get_or_init(|| Arc::new(RwLock::new(EpicGamesClient::default())))
            .clone()
    }
}

#[async_trait]
impl StorefrontClient for EpicGamesClient {
    async fn init(&mut self, mut config: Config) -> Result<()> {
        let refresh_token = config.eg_refresh_token();
        if refresh_token.is_none() {
            return Ok(());
        }

        let services = Services::from_refresh_token(refresh_token.unwrap()).await?;
        let new_refresh_token = services.refresh_token();

        config.set_eg_refresh_token(Some(new_refresh_token))?;

        self.services = Some(Arc::new(services));

        Ok(())
    }
}
