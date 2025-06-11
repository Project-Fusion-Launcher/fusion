use anyhow::Result;
use database::models::{Config, GameSource};
use epic_games::EpicGamesClient;
use gpui::App;
use gpui_tokio::Tokio;
use legacy_games::LegacyGamesClient;
use std::sync::Arc;
use storefront::StorefrontClient;
use strum::IntoEnumIterator;
use tokio::sync::RwLock;

pub fn get_storefront(source: GameSource) -> Arc<RwLock<dyn StorefrontClient + Send + Sync>> {
    match source {
        GameSource::LegacyGames => LegacyGamesClient::get_legacy_games(),
        GameSource::EpicGames => EpicGamesClient::get_epic_games(),
        _ => panic!("Unsupported game source: {:?}", source),
    }
}

pub fn init(app: &mut App) -> Result<()> {
    let config = app.global::<Config>().clone();

    for source in GameSource::iter() {
        if source == GameSource::Itchio {
            continue;
        }
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
