use crate::models::game::GameSource;
use epicgames::EpicGames;
use itchio::Itchio;
use legacygames::LegacyGames;
use std::sync::{Arc, OnceLock};
use storefront::Storefront;
use strum::IntoEnumIterator;
use tokio::{sync::RwLock, task::JoinSet};

pub mod epicgames;
pub mod itchio;
pub mod legacygames;
#[macro_use]
pub mod storefront;

static ITCHIO: OnceLock<Arc<RwLock<dyn Storefront + Send + Sync>>> = OnceLock::new();
static LEGACY_GAMES: OnceLock<Arc<RwLock<dyn Storefront + Send + Sync>>> = OnceLock::new();
static EPIC_GAMES: OnceLock<Arc<RwLock<dyn Storefront + Send + Sync>>> = OnceLock::new();

pub fn get_storefront(source: &GameSource) -> Arc<RwLock<dyn Storefront + Send + Sync>> {
    match source {
        GameSource::Itchio => ITCHIO
            .get_or_init(|| Arc::new(RwLock::new(Itchio::default())))
            .clone(),
        GameSource::LegacyGames => LEGACY_GAMES
            .get_or_init(|| Arc::new(RwLock::new(LegacyGames::default())))
            .clone(),
        GameSource::EpicGames => EPIC_GAMES
            .get_or_init(|| Arc::new(RwLock::new(EpicGames::default())))
            .clone(),
    }
}

pub async fn init_storefronts() -> Result<(), String> {
    let mut tasks = JoinSet::new();

    for source in GameSource::iter() {
        let store = get_storefront(&source);

        tasks.spawn(async move {
            let mut store = store.write().await; // Handle the poison case appropriately
            store.init().await
        });
    }

    while let Some(res) = tasks.join_next().await {
        match res {
            Ok(Ok(_)) => (),
            Ok(Err(e)) => return Err(e.to_string()),
            Err(e) => return Err(e.to_string()),
        }
    }

    Ok(())
}
