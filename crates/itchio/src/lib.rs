use crate::api::services::Services;
use anyhow::Result;
use async_trait::async_trait;
use database::models::Config;
use std::sync::{Arc, OnceLock};
use storefront::StorefrontClient;
use tokio::sync::RwLock;

mod api;

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
}
