use std::path::PathBuf;

use async_trait::async_trait;

use crate::{
    common::result::Result,
    managers::download::{Download, DownloadOptions},
    models::game::{Game, GameVersion, GameVersionInfo},
};

#[async_trait]
pub trait Storefront {
    async fn init(&mut self) -> Result<()>;
    async fn fetch_games(&self) -> Result<Option<Vec<Game>>>;
    async fn fetch_game_versions(&self, game: Game) -> Result<Vec<GameVersion>>;
    async fn fetch_game_version_info(
        &self,
        game: Game,
        version_id: String,
    ) -> Result<GameVersionInfo>;
    async fn pre_download(
        &self,
        game: &mut Game,
        version_id: String,
        download_options: DownloadOptions,
    ) -> Result<Option<Download>>;
    async fn post_download(&self, game_id: &str, path: PathBuf, file_name: &str) -> Result<()>;
    async fn launch_game(&self, game: Game) -> Result<()>;
    async fn uninstall_game(&self, game: &Game) -> Result<()>;
}
