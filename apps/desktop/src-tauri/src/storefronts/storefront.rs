use crate::{
    common::result::Result,
    models::{
        download::{Download, DownloadManifest},
        game::{Game, GameVersion, GameVersionInfo},
        payloads::DownloadOptions,
    },
};
use async_trait::async_trait;
use reqwest::RequestBuilder;
use std::path::PathBuf;

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

    async fn game_manifest(&self, game_id: &str, version_id: &str) -> Result<DownloadManifest>;
    async fn chunk_request(&self, http: &reqwest::Client, url: &str) -> Result<RequestBuilder>;
    async fn process_chunk(&self, path: PathBuf) -> Result<()>;
}
