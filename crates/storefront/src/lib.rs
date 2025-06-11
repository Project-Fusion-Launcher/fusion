use anyhow::Result;
use async_trait::async_trait;
use database::models::{Config, Game};

#[async_trait]
pub trait StorefrontClient {
    async fn init(&mut self, config: Config) -> Result<()>;
    async fn fetch_games(&self) -> Result<Vec<Game>>;
}
