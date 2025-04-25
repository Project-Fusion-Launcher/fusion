use super::storefront::Storefront;
use crate::{
    common::{database, result::Result},
    managers::download::{Download, DownloadOptions},
    models::{
        config::Config,
        game::{Game, GameSource, GameStatus, GameVersion, GameVersionInfo},
    },
    APP,
};
use async_trait::async_trait;
use std::{path::PathBuf, sync::RwLock};
use tauri::Manager;
use wrapper_epicgames::{api::models::KeyImageType, EpicGamesClient};

#[derive(Default)]
pub struct EpicGames {
    client: Option<EpicGamesClient>,
}

#[async_trait]
impl Storefront for EpicGames {
    async fn init(&mut self) -> Result<()> {
        let refresh_token = APP
            .get()
            .unwrap()
            .state::<RwLock<Config>>()
            .read()
            .unwrap()
            .epic_games_refresh_token();

        if refresh_token.is_none() {
            return Ok(());
        }

        let client = EpicGamesClient::from_refresh_token(refresh_token.unwrap()).await?;
        let new_refresh_token = client.refresh_token();

        let mut connection = database::create_connection()?;

        APP.get()
            .unwrap()
            .state::<RwLock<Config>>()
            .write()
            .unwrap()
            .set_epic_games_refresh_token(Some(new_refresh_token), &mut connection)?;

        self.client = Some(client);

        Ok(())
    }

    async fn fetch_games(&self) -> Result<Option<Vec<Game>>> {
        let client = match &self.client {
            Some(c) => c,
            None => return Ok(None),
        };

        let games = client.fetch_games().await?;

        Ok(Some(
            games
                .into_iter()
                .map(|game| Game {
                    id: game.id,
                    title: game.title.clone(),
                    source: GameSource::EpicGames,
                    key: None,
                    developer: Some(game.developer),
                    launch_target: None,
                    path: None,
                    version: None,
                    status: GameStatus::NotInstalled,
                    favorite: false,
                    hidden: false,
                    cover_url: game
                        .key_images
                        .iter()
                        .find(|image| image.image_type == KeyImageType::DieselGameBoxTall)
                        .map(|image| image.url.clone()),
                    sort_title: game.title.to_lowercase(),
                })
                .collect(),
        ))
    }

    async fn fetch_game_versions(&self, game: Game) -> Result<Vec<GameVersion>> {
        let client = match &self.client {
            Some(c) => c,
            None => return Err("Epic Games client not initialized".into()),
        };

        let versions = client
            .fetch_game_versions(&game.id)
            .await
            .map_err(|_| "Failed to fetch game versions")?;

        Ok(versions
            .into_iter()
            .map(|v| GameVersion {
                id: v.clone(),
                name: v,
                external: false,
            })
            .collect())
    }

    async fn fetch_game_version_info(
        &self,
        game: Game,
        version_id: String,
    ) -> Result<GameVersionInfo> {
        let client = match &self.client {
            Some(c) => c,
            None => return Err("Epic Games client not initialized".into()),
        };

        let manifest = client
            .fetch_version_manifest(&game.id, &version_id)
            .await
            .unwrap();

        let download_size = manifest
            .chunk_data_list
            .chunks
            .iter()
            .map(|chunk| chunk.file_size)
            .sum::<u64>();

        let install_size = manifest
            .file_manifest_list
            .elements
            .iter()
            .map(|file| file.file_size)
            .sum::<u64>();

        Ok(GameVersionInfo {
            install_size,
            download_size,
        })
    }

    async fn pre_download(
        &self,
        game: &mut Game,
        _version_id: String,
        download_options: DownloadOptions,
    ) -> Result<Option<Download>> {
        Ok(None)
    }

    async fn launch_game(&self, game: Game) -> Result<()> {
        Ok(())
    }

    async fn uninstall_game(&self, game: &Game) -> Result<()> {
        Ok(())
    }

    async fn post_download(&self, game_id: &str, path: PathBuf, file_name: &str) -> Result<()> {
        Ok(())
    }
}
