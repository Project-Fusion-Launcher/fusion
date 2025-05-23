use super::storefront::Storefront;
use crate::{
    common::{database, result::Result},
    downloads::DownloadStrategy,
    models::{
        config::Config,
        download::{Download, DownloadManifest},
        game::{Game, GameSource, GameStatus, GameVersion, GameVersionInfo},
        payloads::DownloadOptions,
    },
    utils, APP,
};
use async_trait::async_trait;
use reqwest::{header::ETAG, RequestBuilder};
use std::{
    path::{Path, PathBuf},
    sync::{Arc, RwLock},
};
use tauri::Manager;
use tokio::{fs, task::JoinSet};
use wrapper_legacygames::{api::models::Product, LegacyGamesClient};

#[derive(Default)]
pub struct LegacyGames {
    client: Option<Arc<LegacyGamesClient>>,
}

#[async_trait]
impl Storefront for LegacyGames {
    async fn init(&mut self) -> Result<()> {
        let app_handle = APP.get().unwrap();
        let email = app_handle
            .state::<RwLock<Config>>()
            .read()
            .unwrap()
            .legacy_games_email()
            .clone();
        let token = app_handle
            .state::<RwLock<Config>>()
            .read()
            .unwrap()
            .legacy_games_token()
            .clone();

        if email.is_none() {
            return Ok(());
        }

        let client = match token {
            Some(token) => LegacyGamesClient::from_token(email.unwrap(), token),
            None => LegacyGamesClient::from_email(email.unwrap()),
        };

        self.client = Some(Arc::new(client));

        Ok(())
    }

    async fn fetch_games(&self) -> Result<Vec<Game>> {
        let client = match &self.client {
            Some(c) => Arc::clone(c),
            None => return Ok(vec![]),
        };

        let mut join_set = JoinSet::new();

        if client.is_token_client() {
            let token_client = Arc::clone(&client);

            join_set.spawn(async move {
                token_client
                    .fetch_wp_products()
                    .await
                    .map(|products| create_games(products, false))
            });
        }

        join_set.spawn(async move {
            client
                .fetch_giveaway_products()
                .await
                .map(|products| create_games(products, true))
        });

        let mut result = Vec::new();

        while let Some(res) = join_set.join_next().await {
            match res {
                Ok(Ok(games)) => result.extend(games),
                Ok(Err(e)) => return Err(e.into()),
                Err(e) => return Err(e.into()),
            }
        }

        Ok(result)
    }

    async fn fetch_game_versions(&self, game: Game) -> Result<Vec<GameVersion>> {
        #[cfg(unix)]
        return Ok(vec![]);

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
        if self.client.is_none() {
            return Err("Legacy Games client is not initialized".into());
        }

        let client = self.client.as_ref().unwrap();

        let download_size = if let Some(ref key) = game.key {
            client
                .fetch_wp_installer_size(key.parse()?, &game.id)
                .await?
        } else {
            client.fetch_giveaway_installer_size(&game.id).await?
        } as u64;

        // There is no way to fetch the installed size that I know.
        // The game_installed_size in the API's resonse is actually the download size.
        Ok(GameVersionInfo {
            install_size: 0,
            download_size,
        })
    }

    /*async fn pre_download(
        &self,
        game: &mut Game,
        _version_id: String,
        download_options: DownloadOptions,
    ) -> Result<Option<Download>> {
        if self.client.is_none() {
            return Err("Legacy Games client is not initialized".into());
        }

        let client = self.client.as_ref().unwrap();

        let installer_url = if let Some(ref key) = game.key {
            client.fetch_wp_installer(key.parse()?, &game.id).await?
        } else {
            client.fetch_giveaway_installer(&game.id).await?
        };

        let http = reqwest::Client::new();

        // Extract the MD5 hash from the ETag header
        let response = http.head(&installer_url).send().await?;
        let md5 = response
            .headers()
            .get(ETAG)
            .map(|header| header.to_str().unwrap().trim_matches('"').to_string());

        let size = if let Some(ref key) = game.key {
            client
                .fetch_wp_installer_size(key.parse()?, &game.id)
                .await?
        } else {
            client.fetch_giveaway_installer_size(&game.id).await?
        };

        game.version = Some(game.id.clone());

        Ok(None)

        /*Ok(Some(Download {
            request: http.get(installer_url),
            file_name: String::from("setup.exe"),
            download_options,
            game_source: GameSource::LegacyGames,
            game_id: game.id.clone(),
            game_title: game.title.clone(),
            md5,
            download_size: size as u64,
        })) */
    } */

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

    async fn post_download(&self, game_id: &str, path: PathBuf) -> Result<()> {
        let mut entries = fs::read_dir(&path).await?;
        let entry = entries.next_entry().await?;
        let entry = entry.ok_or("No files found in the directory")?;
        let file_name = entry.file_name();

        let file_path = path.join(file_name);

        let mut connection = database::create_connection()?;
        let mut game = Game::select_one(&mut connection, &GameSource::LegacyGames, game_id)?;

        println!("Extracting game: {:?}", file_path);
        utils::file::extract_file(&file_path, &path).await?;

        let mut launch_target = utils::fs::find_launch_target(&path).await?;

        // Strip base path from launch target
        if let Some(target) = &launch_target {
            #[cfg(unix)]
            utils::file::set_permissions(&target, 0o755).await?;
            launch_target = Some(target.strip_prefix(&path).unwrap().to_path_buf());
        }

        game.launch_target = launch_target.map(|target| target.to_string_lossy().into_owned());
        game.status = GameStatus::Installed;
        game.update(&mut connection).unwrap();

        Ok(())
    }

    async fn game_manifest(&self, game_id: &str, version_id: &str) -> Result<DownloadManifest> {
        Err("Not implemented".into())
    }

    fn download_strategy(&self) -> Arc<dyn DownloadStrategy> {
        unimplemented!()
    }
}

fn create_games(products: Vec<Product>, is_giveaway: bool) -> Vec<Game> {
    products
        .into_iter()
        .flat_map(|product| {
            product.games.into_iter().map(move |game| {
                let (game_id, product_id) = if is_giveaway {
                    (game.installer_uuid.to_string(), None)
                } else {
                    (game.game_id.to_string(), Some(product.id.to_string()))
                };

                Game {
                    id: game_id,
                    title: game.game_name.clone(),
                    source: GameSource::LegacyGames,
                    key: product_id,
                    developer: None,
                    launch_target: None,
                    path: None,
                    version: None,
                    status: GameStatus::NotInstalled,
                    favorite: false,
                    hidden: false,
                    cover_url: Some(game.game_coverart),
                    sort_title: game.game_name.to_lowercase(),
                }
            })
        })
        .collect()
}
