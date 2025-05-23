use self::services::Services;
use super::storefront::Storefront;
use crate::{
    common::{database, result::Result},
    downloads::DownloadStrategy,
    models::{
        config::Config,
        download::{Download, DownloadManifest},
        game::{Game, GameSource, GameStatus, GameVersion, GameVersionInfo},
        payloads::DownloadPayload,
    },
    utils, APP,
};
use api::{models::UploadTraits, services};
use async_trait::async_trait;
use std::{
    path::{Path, PathBuf},
    sync::{Arc, RwLock},
    time::Duration,
};
use strategy::ItchioDownload;
use tauri::{webview::DownloadEvent, Emitter, Manager, Url, WebviewUrl, WebviewWindow};
use tokio::{fs, time};

mod api;
mod conversions;
mod strategy;

pub struct Itchio {
    strategy: Arc<dyn DownloadStrategy>,
    services: Option<Arc<Services>>,
}

impl Default for Itchio {
    fn default() -> Self {
        Self {
            strategy: Arc::new(strategy::ItchioStrategy {}),
            services: None,
        }
    }
}

#[async_trait]
impl Storefront for Itchio {
    async fn init(&mut self) -> Result<()> {
        let api_key = APP
            .get()
            .unwrap()
            .state::<RwLock<Config>>()
            .read()
            .unwrap()
            .itchio_api_key();

        let api_key = match api_key {
            Some(key) => key,
            None => return Ok(()),
        };

        self.services = Some(Arc::new(Services::new(api_key)));

        Ok(())
    }

    async fn fetch_games(&self) -> Result<Option<Vec<Game>>> {
        let services = match &self.services {
            Some(s) => s,
            None => return Ok(None),
        };

        let mut games = Vec::new();
        let mut page = 1;

        loop {
            let owned_keys = services.fetch_owned_keys(page).await?;
            let current_page_count = owned_keys.owned_keys.len() as u8;

            games.extend(owned_keys.owned_keys.into_iter().map(Game::from));

            if current_page_count < owned_keys.per_page {
                break;
            }

            page += 1;
        }

        Ok(Some(games))
    }

    async fn fetch_game_versions(&self, game: Game) -> Result<Vec<GameVersion>> {
        let services = match &self.services {
            Some(s) => s,
            None => return Err("itch.io api key not set".into()),
        };

        let game_id: u32 = game.id.parse()?;
        let game_key: u32 = game.key.unwrap().parse()?;
        let uploads = services.fetch_game_uploads(game_id, game_key).await?;

        #[cfg(target_os = "linux")]
        let os_trait = UploadTraits::PLinux;
        #[cfg(target_os = "windows")]
        let os_trait = UploadTraits::PWindows;
        #[cfg(target_os = "macos")]
        let os_trait = UploadTraits::POsx;

        let game_versions = uploads
            .into_iter()
            .filter(|upload| upload.traits.contains(&os_trait))
            .map(GameVersion::from)
            .collect();

        Ok(game_versions)
    }

    async fn fetch_game_version_info(
        &self,
        game: Game,
        version_id: String,
    ) -> Result<GameVersionInfo> {
        let services = match &self.services {
            Some(s) => s,
            None => return Err("itch.io api key not set".into()),
        };

        let upload_id: u32 = version_id.parse()?;
        let game_key: u32 = game.key.unwrap().parse()?;

        let services_clone = Arc::clone(services);

        let upload =
            tokio::spawn(async move { services_clone.fetch_upload(upload_id, game_key).await });

        // Itch.io needs to scan the archive before we can get the info
        let retries = 8;
        for attempt in 1..=retries {
            let scanned_archive = services
                .fetch_upload_scanned_archive(upload_id, game_key)
                .await;

            if let Ok(scanned_archive) = scanned_archive {
                if let Some(install_size) = scanned_archive.extracted_size {
                    return Ok(GameVersionInfo {
                        download_size: upload.await??.size.unwrap_or_default() as u64,
                        install_size: install_size as u64,
                    });
                }
            }

            if attempt < retries {
                time::sleep(Duration::from_secs(attempt * 2)).await;
            }
        }

        Err("Failed to fetch release info".into())
    }

    /*async fn pre_download(
        &self,
        game: &mut Game,
        version_id: String,
        download_options: DownloadOptions,
    ) -> Result<Option<Download>> {
        let client = match &self.client {
            Some(c) => c,
            None => return Err("itch.io client is not initialized".into()),
        };

        let upload_id: u32 = version_id.parse()?;
        let game_key: u32 = game.key.clone().unwrap().parse()?;

        let upload = client.fetch_game_upload(upload_id, game_key).await?;

        let download_request = client.fetch_upload_download_url(upload_id, game_key);

        if upload.storage == UploadStorage::External {
            let response = download_request.send().await?;
            let url = response.url().to_owned();
            handle_external_download(
                url,
                &download_options.install_location,
                &game.id,
                &game.title,
            )
            .await?;
            return Ok(None);
        }

        game.version = upload
            .build
            .as_ref()
            .map(|build| build.version.to_string())
            .or(upload.md5_hash.clone());

        Ok(None)

        /*Ok(Some(Download {
            request: download_request,
            file_name: upload.filename,
            download_options,
            game_source: GameSource::Itchio,
            game_id: game.id.clone(),
            game_title: game.title.clone(),
            md5: upload.md5_hash,
            download_size: upload.size.unwrap_or(0) as u64,
        })) */
    }*/

    async fn launch_game(&self, game: Game) -> Result<()> {
        let game_path = game.path.unwrap();
        let launch_target = game.launch_target.unwrap();

        let target_path = PathBuf::from(&game_path).join(&launch_target);

        utils::file::execute_file(&target_path)?;

        Ok(())
    }

    async fn uninstall_game(&self, game: &Game) -> Result<()> {
        let path = PathBuf::from(game.path.as_ref().unwrap());

        if path.exists() {
            fs::remove_dir_all(&path).await?;
        }

        Ok(())
    }

    async fn post_download(&self, game_id: &str, path: PathBuf) -> Result<()> {
        post_download(game_id, path).await
    }

    async fn game_manifest(&self, _game_id: &str, _version_id: &str) -> Result<DownloadManifest> {
        Err("Not implemented".into())
    }

    fn download_strategy(&self) -> Arc<dyn DownloadStrategy> {
        Arc::clone(&self.strategy)
    }
}

impl Itchio {
    pub(self) async fn fetch_download_info(&self, download: &Download) -> Result<ItchioDownload> {
        let services = match &self.services {
            Some(s) => s,
            None => return Err("itch.io api key not set".into()),
        };

        let mut connection = database::create_connection()?;
        let game = Game::select_one(&mut connection, &download.game_source, &download.game_id)?;

        let upload_id: u32 = download.game_version_id.parse()?;
        let game_key: u32 = game.key.clone().unwrap().parse()?;

        let upload = services.fetch_upload(upload_id, game_key).await?;

        let download_request = services.fetch_upload_download(upload_id, game_key);

        Ok(ItchioDownload {
            request: download_request,
            filename: upload.filename,
            md5: upload.md5_hash,
        })
    }
}

async fn post_download(game_id: &str, path: PathBuf) -> Result<()> {
    let mut entries = fs::read_dir(&path).await?;
    let entry = entries.next_entry().await?;
    let entry = entry.ok_or("No files found in the directory")?;
    let file_name = entry.file_name();

    let file_path = path.join(file_name);

    let mut connection = database::create_connection()?;
    let mut game = Game::select_one(&mut connection, &GameSource::Itchio, game_id)?;

    if file_path.extension().unwrap() == "zip"
        || file_path.extension().unwrap() == "7z"
        || file_path.extension().unwrap() == "rar"
    {
        println!("Extracting game: {:?}", file_path);
        utils::file::extract_file(&file_path, &path).await?;
    }

    let mut launch_target = utils::fs::find_launch_target(&path).await?;

    println!("Launch target: {:?}", launch_target);

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

async fn post_download_external(
    game_id: &str,
    path: PathBuf,
    file_name: &str,
    success: bool,
) -> Result<()> {
    if !success {
        // TODO: handle download failure
        return Err("Download failed".into());
    }

    let file_path = path.join(file_name);
    let size = fs::metadata(&file_path).await?.len();

    let mut connection = database::create_connection()?;
    let game = Game::select_one(&mut connection, &GameSource::Itchio, game_id)?;

    let payload = DownloadPayload {
        game_id: game.id,
        game_source: GameSource::Itchio,
        game_title: game.title,
        download_size: size,
        downloaded: size,
    };

    APP.get()
        .unwrap()
        .emit("download-finished", &payload)
        .unwrap();

    post_download(game_id, path).await?;

    APP.get()
        .unwrap()
        .emit("download-installed", &payload)
        .unwrap();

    Ok(())
}

async fn handle_external_download(
    url: Url,
    install_location: &Path,
    game_id: &str,
    game_title: &str,
) -> Result<()> {
    if APP
        .get()
        .unwrap()
        .get_webview_window("ichio-external")
        .is_some()
    {
        return Err("External download window already open, wait for the download to finish before starting a new one".into());
    }

    let install_location = install_location.to_path_buf();
    let game_id = game_id.to_string();
    let game_title = game_title.to_string();
    WebviewWindow::builder(
        APP.get().unwrap(),
        "itchio-external",
        WebviewUrl::External(url),
    )
    .on_download(move |webview, event| {
        match event {
            DownloadEvent::Requested { url, destination } => {
                webview.window().hide().unwrap();
                println!("downloading {}", url);

                let mut connection = database::create_connection().unwrap();
                let mut game =
                    Game::select_one(&mut connection, &GameSource::Itchio, &game_id).unwrap();

                game.status = GameStatus::Downloading;
                game.update(&mut connection).unwrap();

                *destination = install_location
                    .clone()
                    .join(destination.file_name().unwrap());

                APP.get()
                    .unwrap()
                    .emit(
                        "download-external",
                        DownloadPayload {
                            game_id: game_id.clone(),
                            game_source: GameSource::Itchio,
                            game_title: game_title.clone(),
                            download_size: 0,
                            downloaded: 0,
                        },
                    )
                    .unwrap();
            }
            DownloadEvent::Finished { url, path, success } => {
                println!("downloaded {} to {:?}, success: {}", url, path, success);

                let path = path.unwrap();
                let file_name = path.file_name().unwrap().to_string_lossy().into_owned();
                let directory = path.parent().unwrap().to_path_buf();

                let game_id = game_id.clone();
                tokio::spawn(async move {
                    if let Err(e) =
                        post_download_external(&game_id, directory, &file_name, success).await
                    {
                        eprintln!("Failed to post download: {}", e);
                    }
                });
                webview.window().close().unwrap();
            }
            _ => (),
        }
        // let the download start
        true
    })
    .initialization_script(
        r#"
            // Override window.open
            window.open = function (url, ...args) {
                window.location.href = url;
                return null;
            };

            // Intercept links with target="_blank"
            document.addEventListener('click', function (event) {
                const link = event.target.closest('a');
                if (link && link.target === '_blank') {
                    event.preventDefault();
                    window.location.href = link.href;
                }
            });
        "#,
    )
    .title("itch.io External Download")
    .build()?;
    Ok(())
}
