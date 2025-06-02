use super::{DownloadStrategy, Storefront};
use crate::{
    common::result::Result,
    models::{
        config::Config,
        download::Download,
        game::{Game, GameStatus, GameVersion, GameVersionInfo},
    },
    utils::{self, launch_target},
    APP,
};
use api::services::Services;
use async_trait::async_trait;
use reqwest::header::ETAG;
use std::{
    path::PathBuf,
    sync::{Arc, RwLock},
};
use strategy::LegacyGamesDownload;
use tauri::Manager;
use tokio::fs;

mod api;
mod conversions;
mod strategy;

pub struct LegacyGames {
    services: Option<Services>,
    strategy: Arc<dyn DownloadStrategy>,
}

impl Default for LegacyGames {
    fn default() -> Self {
        Self {
            services: None,
            strategy: Arc::new(strategy::LegacyGamesStrategy {}),
        }
    }
}

#[async_trait]
impl Storefront for LegacyGames {
    async fn init(&mut self) -> Result<()> {
        let (email, token) = {
            let config_lock = APP.get().unwrap().state::<RwLock<Config>>();
            let config = config_lock.read().unwrap();
            (
                config.legacy_games_email().clone(),
                config.legacy_games_token().clone(),
            )
        };

        if email.is_none() {
            return Ok(());
        }

        let services = match token {
            Some(token) => Services::from_token(email.unwrap(), token).await?,
            None => Services::from_email(email.unwrap()).await?,
        };

        self.services = Some(services);

        Ok(())
    }

    async fn fetch_games(&self) -> Result<Vec<Game>> {
        let services = match &self.services {
            Some(c) => c,
            None => return Ok(vec![]),
        };

        let products = services.fetch_products().await?;
        let games = products.into_iter().flat_map(Vec::<Game>::from).collect();

        Ok(games)
    }

    async fn fetch_game_versions(&self, game: Game) -> Result<Vec<GameVersion>> {
        /*#[cfg(unix)]
        return Ok(vec![]);*/

        Ok(vec![GameVersion {
            id: game.id.clone(),
            name: game.title,
            external: false,
        }])
    }

    async fn fetch_game_version_info(
        &self,
        game: Game,
        _version_id: String,
    ) -> Result<GameVersionInfo> {
        let services = match &self.services {
            Some(c) => c,
            None => return Err("Legacy Games services not initialized".into()),
        };

        let download_size = if let Some(ref key) = game.key {
            services
                .fetch_wp_installer_size(key.parse()?, &game.id)
                .await?
        } else {
            services.fetch_giveaway_installer_size(&game.id).await?
        } as u64;

        // There is no way to fetch the installed size that I know.
        // The game_installed_size in the API's resonse is actually the download size.
        Ok(GameVersionInfo {
            install_size: 0,
            download_size,
        })
    }

    async fn launch_game(&self, game: Game) -> Result<()> {
        let game_path = game.path.unwrap();
        let launch_target = game.launch_target.unwrap();

        let target_path = PathBuf::from(&game_path).join(&launch_target);

        utils::file::execute_file(&target_path)?;

        Ok(())
    }

    async fn uninstall_game(&self, game: &Game) -> Result<()> {
        let path = PathBuf::from(game.path.as_ref().unwrap());

        // The uninstaller requires admin. Removing the directory should be enough
        // as nothing is created in the registry (we also don't use the installer).
        if path.exists() {
            fs::remove_dir_all(&path).await?;
        }

        Ok(())
    }

    async fn post_download(&self, game: &mut Game, path: PathBuf) -> Result<()> {
        let mut entries = fs::read_dir(&path).await?;
        let entry = entries.next_entry().await?;
        let entry = entry.ok_or("No files found in the directory")?;
        let file_name = entry.file_name();

        let file_path = path.join(file_name);

        println!("Extracting game: {:?}", file_path);
        utils::file::extract_file(&file_path, &path).await?;

        let mut launch_target = launch_target::find_launch_target(&path).await?;

        // Strip base path from launch target
        if let Some(target) = &launch_target {
            #[cfg(unix)]
            utils::file::set_permissions(&target, 0o755).await?;
            launch_target = Some(target.strip_prefix(&path).unwrap().to_path_buf());
        }

        game.launch_target = launch_target.map(|target| target.to_string_lossy().into_owned());
        game.status = GameStatus::Installed;
        game.update()?;

        Ok(())
    }

    fn download_strategy(&self) -> Arc<dyn DownloadStrategy> {
        Arc::clone(&self.strategy)
    }
}

impl LegacyGames {
    pub(self) async fn fetch_download_info(
        &self,
        download: &Download,
    ) -> Result<LegacyGamesDownload> {
        let services = match &self.services {
            Some(c) => c,
            None => return Err("Legacy Games services not initialized".into()),
        };

        let game = Game::select_one(&download.game_id, &download.game_source)?;

        let url = if let Some(ref key) = game.key {
            services.fetch_wp_installer(key.parse()?, &game.id).await?
        } else {
            services.fetch_giveaway_installer(&game.id).await?
        };

        let http = reqwest::Client::new();

        // Extract the MD5 hash from the ETag header
        let response = http.head(&url).send().await?;
        let md5 = response
            .headers()
            .get(ETAG)
            .map(|header| header.to_str().unwrap().trim_matches('"').to_string());

        Ok(LegacyGamesDownload {
            request: http.get(url),
            filename: String::from("setup.exe"),
            md5,
        })
    }
}
