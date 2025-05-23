use crate::{
    common::result::Result,
    downloads::DownloadStrategy,
    models::{
        download::DownloadManifest,
        game::{Game, GameVersion, GameVersionInfo},
    },
};
use async_trait::async_trait;
use std::{path::PathBuf, sync::Arc};

#[async_trait]
pub trait Storefront {
    async fn init(&mut self) -> Result<()>;
    async fn fetch_games(&self) -> Result<Vec<Game>>;
    async fn fetch_game_versions(&self, game: Game) -> Result<Vec<GameVersion>>;
    async fn fetch_game_version_info(
        &self,
        game: Game,
        version_id: String,
    ) -> Result<GameVersionInfo>;
    fn download_strategy(&self) -> Arc<dyn DownloadStrategy>;
    async fn post_download(&self, game_id: &str, path: PathBuf) -> Result<()>;
    async fn launch_game(&self, game: Game) -> Result<()>;
    async fn uninstall_game(&self, game: &Game) -> Result<()>;

    async fn game_manifest(&self, game_id: &str, version_id: &str) -> Result<DownloadManifest>;
}
