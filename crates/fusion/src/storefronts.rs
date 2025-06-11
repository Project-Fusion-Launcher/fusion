use anyhow::Result;
use database::models::{Config, Game, GameSource};
use epic_games::EpicGamesClient;
use gpui::App;
use gpui_tokio::Tokio;
use itchio::ItchioClient;
use legacy_games::LegacyGamesClient;
use std::sync::Arc;
use storefront::StorefrontClient;
use strum::IntoEnumIterator;
use tokio::{sync::RwLock, task::JoinSet};

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

pub async fn get_games(refetch: bool) -> Result<Vec<Game>> {
    if refetch {
        let mut tasks = JoinSet::new();
        let mut games_to_insert = Vec::new();

        for source in GameSource::iter() {
            let store = get_storefront(source);
            tasks.spawn(async move { store.read().await.fetch_games().await });
        }

        while let Some(res) = tasks.join_next().await {
            match res {
                Ok(fetched_games) => match fetched_games {
                    Ok(fetched_games) => games_to_insert.extend(fetched_games),
                    Err(e) => println!("{:?}", e),
                },
                Err(e) => println!("{:?}", e),
            }
        }

        Game::insert_or_ignore(&games_to_insert)?;
    }

    Game::all()
}
