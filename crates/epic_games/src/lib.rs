use crate::api::{models::CategoryPath, services::Services};
use anyhow::Result;
use async_trait::async_trait;
use database::models::{Config, Game};
use std::sync::{Arc, OnceLock};
use storefront::StorefrontClient;
use tokio::{
    sync::{RwLock, mpsc},
    task,
};
use worker::WorkerPool;

mod api;
mod conversions;

static EPIC_GAMES: OnceLock<Arc<RwLock<EpicGamesClient>>> = OnceLock::new();

#[derive(Default)]
pub struct EpicGamesClient {
    services: Option<Arc<Services>>,
}

impl EpicGamesClient {
    pub fn get_client() -> Arc<RwLock<EpicGamesClient>> {
        EPIC_GAMES
            .get_or_init(|| Arc::new(RwLock::new(EpicGamesClient::default())))
            .clone()
    }
}

#[async_trait]
impl StorefrontClient for EpicGamesClient {
    async fn init(&mut self, mut config: Config) -> Result<()> {
        let refresh_token = config.eg_refresh_token();
        if refresh_token.is_none() {
            return Ok(());
        }

        let services = Services::from_refresh_token(refresh_token.unwrap()).await?;
        let new_refresh_token = services.refresh_token();

        config.set_eg_refresh_token(Some(new_refresh_token))?;

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
                } else {
                    eprintln!("{:?}", game);
                }
            })
            .await?;
        }

        drop(tx);

        pool.shutdown().await;
        Ok(result.await?)
    }
}
