use crate::{
    common::result::Result,
    models::{download::Download, game::*},
};
use async_trait::async_trait;
use epicgames::EpicGames;
use itchio::Itchio;
use legacygames::LegacyGames;
use std::{
    path::PathBuf,
    sync::{Arc, OnceLock},
};
use strum::IntoEnumIterator;
use tokio::{sync::RwLock, task::JoinSet};
use tokio_util::sync::CancellationToken;

mod epicgames;
mod itchio;
mod legacygames;

static ITCHIO: OnceLock<Arc<RwLock<Itchio>>> = OnceLock::new();
static LEGACY_GAMES: OnceLock<Arc<RwLock<LegacyGames>>> = OnceLock::new();
static EPIC_GAMES: OnceLock<Arc<RwLock<EpicGames>>> = OnceLock::new();

#[async_trait]
pub trait Storefront {
    async fn init(&mut self) -> Result<()>;
    async fn fetch_games(&self) -> Result<Vec<Game>>;
    async fn fetch_game_versions(&self, game: &Game) -> Result<Vec<GameVersion>>;
    async fn fetch_game_version_info(
        &self,
        game: &Game,
        version_id: String,
    ) -> Result<GameVersionInfo>;
    fn download_strategy(&self) -> Arc<dyn DownloadStrategy>;
    async fn post_download(&self, game: &mut Game, path: PathBuf) -> Result<()>;
    async fn launch_game(&self, game: Game) -> Result<()>;
    async fn uninstall_game(&self, game: &Game) -> Result<()>;
}

#[async_trait]
pub trait DownloadStrategy: Send + Sync {
    async fn start(
        &self,
        download: Arc<Download>,
        cancellation_token: CancellationToken,
    ) -> Result<bool>;
}

pub fn get_storefront(source: GameSource) -> Arc<RwLock<dyn Storefront + Send + Sync>> {
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

pub fn init_storefronts() {
    tokio::task::spawn(async {
        let mut tasks = JoinSet::new();

        for source in GameSource::iter() {
            let store = get_storefront(source);

            tasks.spawn(async move {
                let mut store = store.write().await;
                store.init().await
            });
        }

        while let Some(res) = tasks.join_next().await {
            match res {
                Ok(Ok(_)) => (),
                Ok(Err(e)) => panic!("Error initializing storefront: {:?}", e),
                Err(e) => panic!("Error joining task: {:?}", e),
            }
        }
    });
}

fn get_or_init_store<T: Default + 'static>(
    cell: &'static OnceLock<Arc<RwLock<T>>>,
) -> Arc<RwLock<T>> {
    cell.get_or_init(|| Arc::new(RwLock::new(T::default())))
        .clone()
}
