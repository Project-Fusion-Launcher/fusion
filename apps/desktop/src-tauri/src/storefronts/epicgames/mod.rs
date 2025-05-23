use super::storefront::Storefront;
use crate::{
    common::{database, result::Result, worker::WorkerPool},
    downloads::DownloadStrategy,
    models::{config::Config, download::*, game::*},
    utils::string,
    APP,
};
use api::{models::CategoryPath, services::Services};
use async_trait::async_trait;
use std::{
    path::PathBuf,
    sync::{Arc, RwLock},
};
use strategy::EpicGamesStrategy;
use tauri::Manager;
use tokio::{sync::mpsc, task};
use wrapper_epicgames::EpicGamesClient;

mod api;
mod conversions;
mod strategy;

pub struct EpicGames {
    client: Option<EpicGamesClient>,
    services: Option<Arc<Services>>,
    strategy: Arc<dyn DownloadStrategy>,
}

impl Default for EpicGames {
    fn default() -> Self {
        Self {
            client: None,
            services: None,
            strategy: Arc::new(EpicGamesStrategy {}),
        }
    }
}

#[async_trait]
impl Storefront for EpicGames {
    async fn init(&mut self) -> Result<()> {
        let config_lock = APP.get().unwrap().state::<RwLock<Config>>();

        let refresh_token = config_lock.read().unwrap().epic_games_refresh_token();
        if refresh_token.is_none() {
            return Ok(());
        }

        let services = Services::from_refresh_token(refresh_token.unwrap()).await?;
        let new_refresh_token = services.refresh_token();

        let mut connection = database::create_connection()?;
        config_lock
            .write()
            .unwrap()
            .set_epic_games_refresh_token(Some(new_refresh_token), &mut connection)?;

        self.client = None;
        self.services = Some(Arc::new(services));

        Ok(())
    }

    async fn fetch_games(&self) -> Result<Vec<Game>> {
        let services = match &self.services {
            Some(c) => c,
            None => return Ok(vec![]),
        };

        let assets = services.fetch_game_assets("Windows").await?;
        let pool = WorkerPool::new(16);
        let (tx, mut rx) = mpsc::channel::<api::models::Game>(24);

        let result = task::spawn(async move {
            let mut games: Vec<Game> = vec![];

            while let Some(game) = rx.recv().await {
                if game.main_game_item.is_none()
                    && game
                        .categories
                        .iter()
                        .any(|c| c.path == CategoryPath::Games)
                {
                    games.push(Game::from(game));
                }
            }

            games
        });

        for asset in assets {
            if asset.namespace == "ue" {
                continue;
            }

            let services = Arc::clone(services);
            let tx = tx.clone();

            pool.execute(move || async move {
                let game = services
                    .fetch_game_info(&asset.namespace, &asset.catalog_item_id)
                    .await;

                if let Ok(game) = game {
                    if tx.send(game).await.is_err() {
                        eprintln!("The receiver dropped");
                    }
                }
            })
            .await?;
        }

        drop(tx);

        pool.shutdown().await;
        Ok(result.await?)
    }

    async fn fetch_game_versions(&self, game: Game) -> Result<Vec<GameVersion>> {
        let services = match &self.services {
            Some(c) => c,
            None => return Err("Epic Games client not initialized".into()),
        };

        let assets = services.fetch_game_assets("Windows").await?;
        let asset = assets
            .into_iter()
            .find(|asset| asset.catalog_item_id == game.id)
            .ok_or("Game not found")?;

        Ok(vec![GameVersion::from(asset)])
    }

    async fn fetch_game_version_info(
        &self,
        game: Game,
        version_id: String,
    ) -> Result<GameVersionInfo> {
        let services = match &self.services {
            Some(c) => c,
            None => return Err("Epic Games client not initialized".into()),
        };

        let assets = services.fetch_game_assets("Windows").await?;
        let asset = assets
            .into_iter()
            .find(|asset| asset.catalog_item_id == game.id)
            .ok_or("Game not found")?;

        let manifest = services
            .fetch_game_manifest(
                "Windows",
                &asset.namespace,
                &asset.catalog_item_id,
                &asset.app_name,
                &version_id,
            )
            .await?;

        /*let download_size = manifest
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
            .sum::<u64>(); */

        Ok(GameVersionInfo {
            install_size: 0,
            download_size: 0,
        })
    }

    /*async fn pre_download(
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
    }*/

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
