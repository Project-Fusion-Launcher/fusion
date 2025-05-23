use crate::models::game::GameSource;
use epicgames::EpicGames;
use itchio::Itchio;
use legacygames::LegacyGames;
use std::sync::{Arc, OnceLock};
use storefront::Storefront;
use strum::IntoEnumIterator;
use tokio::{sync::RwLock, task::JoinSet};

mod epicgames;
mod itchio;
mod legacygames;
#[macro_use]
mod storefront;

static ITCHIO: OnceLock<Arc<RwLock<Itchio>>> = OnceLock::new();
static LEGACY_GAMES: OnceLock<Arc<RwLock<LegacyGames>>> = OnceLock::new();
static EPIC_GAMES: OnceLock<Arc<RwLock<EpicGames>>> = OnceLock::new();

pub fn get_storefront(source: &GameSource) -> Arc<RwLock<dyn Storefront + Send + Sync>> {
    match source {
        GameSource::Itchio => get_itchio(),
        GameSource::LegacyGames => get_legacy_games(),
        GameSource::EpicGames => get_epic_games(),
    }
}

fn get_itchio() -> Arc<RwLock<Itchio>> {
    get_or_init_store(&ITCHIO)
}

fn get_legacy_games() -> Arc<RwLock<LegacyGames>> {
    get_or_init_store(&LEGACY_GAMES)
}

fn get_epic_games() -> Arc<RwLock<EpicGames>> {
    get_or_init_store(&EPIC_GAMES)
}

pub async fn init_storefronts() -> Result<(), String> {
    let mut tasks = JoinSet::new();

    for source in GameSource::iter() {
        let store = get_storefront(&source);

        tasks.spawn(async move {
            let mut store = store.write().await;
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

fn get_or_init_store<T: Default + 'static>(
    cell: &'static OnceLock<Arc<RwLock<T>>>,
) -> Arc<RwLock<T>> {
    cell.get_or_init(|| Arc::new(RwLock::new(T::default())))
        .clone()
}
