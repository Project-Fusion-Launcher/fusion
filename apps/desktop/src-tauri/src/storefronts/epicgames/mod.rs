use super::storefront::Storefront;
use crate::{
    common::{database, result::Result},
    downloads::DownloadStrategy,
    models::{config::Config, download::*, game::*, payloads::DownloadOptions},
    util::string,
    APP,
};
use async_trait::async_trait;
use std::{
    path::PathBuf,
    sync::{Arc, RwLock},
};
use tauri::Manager;
use wrapper_epicgames::{api::models::KeyImageType, EpicGamesClient};

mod strategy;

pub struct EpicGames {
    client: Option<EpicGamesClient>,
    strategy: Arc<dyn DownloadStrategy>,
}

impl Default for EpicGames {
    fn default() -> Self {
        Self {
            client: None,
            strategy: Arc::new(strategy::EpicGamesStrategy {}),
        }
    }
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
            .map(|chunk| chunk.file_size as u64)
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
        version_id: String,
        download_options: DownloadOptions,
    ) -> Result<Option<Download>> {
        Ok(None)
        /*let client = match &self.client {
            Some(c) => c,
            None => return Err("Epic Games client not initialized".into()),
        };

        let manifest = client
            .fetch_version_manifest(&game.id, &version_id)
            .await
            .unwrap();

        let urls = client.fetch_cdn_urls(&game.id, &version_id).await.unwrap();

        let mut download = Download {
            chunks: vec![],
            files: vec![],
            path: download_options.install_location,
            game_id: game.id.clone(),
            game_source: GameSource::EpicGames,
            game_title: game.title.clone(),
        };

        for chunk in manifest.chunk_data_list.chunks.iter() {
            let url = format!("{}/{}", urls[0], chunk.path());

            download.chunks.push(DownloadChunk {
                id: chunk.guid_num(),
                completed: false,
                url,
                compressed_size: chunk.file_size as u64,
                size: chunk.window_size as u64,
                hash: DownloadHash::Sha1(string::array_to_hex(chunk.sha_hash)),
            })
        }

        for file in manifest.file_manifest_list.elements {
            let mut download_file = DownloadFile {
                filename: file.filename,
                hash: DownloadHash::Sha1(string::array_to_hex(file.hash)),
                chunk_parts: vec![],
            };

            for chunk_part in file.chunk_parts.iter() {
                download_file.chunk_parts.push(DownloadChunkPart {
                    id: chunk_part.guid_num(),
                    chunk_offset: chunk_part.offset as u64,
                    file_offset: chunk_part.file_offset,
                    size: chunk_part.size as u64,
                    completed: false,
                })
            }

            download.files.push(download_file);
        }

        Ok(Some(download))*/
    }

    async fn launch_game(&self, _game: Game) -> Result<()> {
        Ok(())
    }

    async fn uninstall_game(&self, _game: &Game) -> Result<()> {
        Ok(())
    }

    async fn post_download(&self, _game_id: &str, _path: PathBuf) -> Result<()> {
        Ok(())
    }

    async fn game_manifest(&self, game_id: &str, version_id: &str) -> Result<DownloadManifest> {
        let client = match &self.client {
            Some(c) => c,
            None => return Err("Epic Games client not initialized".into()),
        };

        let manifest = client
            .fetch_version_manifest(&game_id, &version_id)
            .await
            .unwrap();

        let urls = client.fetch_cdn_urls(&game_id, &version_id).await.unwrap();

        let mut result = DownloadManifest {
            chunks: vec![],
            files: vec![],
        };

        for chunk in manifest.chunk_data_list.chunks.iter() {
            let url = format!("{}/{}", urls[0], chunk.path());

            result.chunks.push(DownloadChunk {
                id: chunk.guid_num(),
                completed: false,
                url,
                compressed_size: chunk.file_size as u64,
                size: chunk.window_size as u64,
                hash: DownloadHash::Sha1(string::array_to_hex(chunk.sha_hash)),
            })
        }

        for file in manifest.file_manifest_list.elements {
            let mut download_file = DownloadFile {
                filename: file.filename,
                hash: DownloadHash::Sha1(string::array_to_hex(file.hash)),
                chunk_parts: vec![],
            };

            for chunk_part in file.chunk_parts.iter() {
                download_file.chunk_parts.push(DownloadChunkPart {
                    id: chunk_part.guid_num(),
                    chunk_offset: chunk_part.offset as u64,
                    file_offset: chunk_part.file_offset,
                    size: chunk_part.size as u64,
                    completed: false,
                })
            }

            result.files.push(download_file);
        }

        Ok(result)
    }

    fn download_strategy(&self) -> Arc<dyn DownloadStrategy> {
        Arc::clone(&self.strategy)
    }
}
