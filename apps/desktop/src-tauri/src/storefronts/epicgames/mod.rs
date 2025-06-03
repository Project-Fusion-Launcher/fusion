use super::{DownloadStrategy, Storefront};
use crate::{
    common::{result::Result, worker::WorkerPool},
    models::{config::Config, game::*},
    storefronts::epicgames::download::{download_plan::DownloadPlan, strategy::EpicGamesStrategy},
    APP,
};
use api::{
    models::{CategoryPath, Manifest},
    services::Services,
};
use async_trait::async_trait;
use reqwest::Url;
use std::{
    path::PathBuf,
    sync::{Arc, RwLock},
};
use tauri::Manager;
use tokio::{
    sync::{mpsc, oneshot},
    task,
};

mod api;
mod conversions;
mod download;

pub struct EpicGames {
    services: Option<Arc<Services>>,
    strategy: Arc<dyn DownloadStrategy>,
}

impl Default for EpicGames {
    fn default() -> Self {
        Self {
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

        config_lock
            .write()
            .unwrap()
            .set_epic_games_refresh_token(Some(new_refresh_token))?;

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

    async fn fetch_game_versions(&self, game: &Game) -> Result<Vec<GameVersion>> {
        let services = match &self.services {
            Some(c) => c,
            None => return Err("Epic Games client not initialized".into()),
        };

        let assets = services.fetch_game_assets("Windows").await?;
        let asset = assets
            .into_iter()
            .find(|asset| &asset.catalog_item_id == game.id())
            .ok_or("Game not found")?;

        Ok(vec![GameVersion::from(asset)])
    }

    async fn fetch_game_version_info(
        &self,
        game: &Game,
        _version_id: String,
    ) -> Result<GameVersionInfo> {
        let manifest = self.get_game_manifest(game.id()).await?;

        let install_size = manifest.install_size();
        let download_size = manifest.download_size();

        Ok(GameVersionInfo {
            install_size,
            download_size,
        })
    }

    async fn launch_game(&self, _game: Game) -> Result<()> {
        Ok(())
    }

    async fn uninstall_game(&self, _game: &Game) -> Result<()> {
        Ok(())
    }

    async fn post_download(&self, _game: &mut Game, _path: PathBuf) -> Result<()> {
        Ok(())
    }

    fn download_strategy(&self) -> Arc<dyn DownloadStrategy> {
        Arc::clone(&self.strategy)
    }
}

impl EpicGames {
    pub async fn get_cdn_url(&self, game_id: &str) -> Result<Url> {
        let services = match &self.services {
            Some(c) => c,
            None => return Err("Epic Games client not initialized".into()),
        };

        let assets = services.fetch_game_assets("Windows").await?;
        let asset = assets
            .into_iter()
            .find(|asset| asset.catalog_item_id == game_id)
            .ok_or("Game not found")?;

        let urls = services
            .fetch_cdn_urls(
                "Windows",
                &asset.namespace,
                &asset.catalog_item_id,
                &asset.app_name,
            )
            .await?;

        let url = urls.first().ok_or("No CDN URL found").cloned()?;
        Ok(url)
    }

    pub async fn get_game_manifest(&self, game_id: &str) -> Result<Manifest> {
        let services = match &self.services {
            Some(c) => c,
            None => return Err("Epic Games client not initialized".into()),
        };

        let assets = services.fetch_game_assets("Windows").await?;
        let asset = assets
            .into_iter()
            .find(|asset| asset.catalog_item_id == game_id)
            .ok_or("Game not found")?;

        services
            .fetch_game_manifest(
                "Windows",
                &asset.namespace,
                &asset.catalog_item_id,
                &asset.app_name,
            )
            .await
    }

    pub async fn compute_download_plan(&self, game_id: &str) -> Result<DownloadPlan> {
        let manifest = self.get_game_manifest(game_id).await?;

        let (tx, rx) = oneshot::channel();

        rayon::spawn(move || {
            let mut plan = DownloadPlan::new(manifest);
            plan.compute();
            let _ = tx.send(plan);
        });

        let plan = rx.await?;

        Ok(plan)
    }
}
