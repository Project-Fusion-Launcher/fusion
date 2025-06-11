use anyhow::Result;
use database::models::{Config, GameSource};
use epic_games::EpicGamesClient;
use gpui::App;
use gpui_tokio::Tokio;
use itchio::ItchioClient;
use legacy_games::LegacyGamesClient;
use std::sync::Arc;
use storefront::StorefrontClient;
use strum::IntoEnumIterator;
use tokio::sync::RwLock;

pub fn get_storefront(source: GameSource) -> Arc<RwLock<dyn StorefrontClient + Send + Sync>> {
    match source {
        GameSource::LegacyGames => LegacyGamesClient::get_client(),
        GameSource::EpicGames => EpicGamesClient::get_client(),
        GameSource::Itchio => ItchioClient::get_client(),
    }
}

pub fn init(app: &mut App) -> Result<()> {
    let config = app.global::<Config>().clone();

    for source in GameSource::iter() {
        let storefront = get_storefront(source);

        let config_clone = config.clone();

        Tokio::spawn(app, async move {
            let mut storefront = storefront.write().await;
            storefront.init(config_clone).await.unwrap();
        })
        .detach();
    }

    Ok(())
}
