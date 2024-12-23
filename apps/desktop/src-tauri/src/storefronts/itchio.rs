use crate::{
    common::{database, result::Result},
    managers::download::{Download, DownloadOptions},
    models::{
        game::{Game, GameSource, GameStatus, GameVersion, VersionDownloadInfo},
        payloads::DownloadPayload,
    },
    util, APP,
};
use std::path::{Path, PathBuf};
use tauri::{webview::DownloadEvent, Emitter, Manager, Url, WebviewUrl, WebviewWindow};
use tokio::fs;
use wrapper_itchio::{api::models::UploadStorage, ItchioClient};

pub async fn fetch_games(api_key: &str) -> Result<Vec<Game>> {
    let client = ItchioClient::new(api_key);
    let mut games = Vec::new();
    let mut page = 1;

    loop {
        let owned_keys = client.fetch_owned_keys(page).await?;
        let current_page_count = owned_keys.owned_keys.len() as u8;

        games.extend(owned_keys.owned_keys.into_iter().map(|key| {
            let developer = key
                .game
                .user
                .and_then(|user| user.display_name.or(Some(user.username)));

            Game {
                id: key.game.id.to_string(),
                title: key.game.title.clone(),
                source: GameSource::Itchio,
                key: Some(key.id.to_string()),
                developer,
                launch_target: None,
                path: None,
                version: None,
                status: GameStatus::NotInstalled,
                favorite: false,
                hidden: false,
                cover_url: key.game.still_cover_url.or(key.game.cover_url),
                sort_title: Some(key.game.title.to_lowercase()),
            }
        }));

        if current_page_count < owned_keys.per_page {
            break;
        }

        page += 1;
    }

    Ok(games)
}

pub async fn fetch_game_versions(
    api_key: &str,
    game_id: &str,
    game_key: &str,
) -> Result<Vec<GameVersion>> {
    let client = ItchioClient::new(api_key);

    let game_id: u32 = game_id.parse()?;
    let game_key: u32 = game_key.parse()?;
    let uploads = client.fetch_game_uploads(game_id, game_key).await?;

    let game_versions = uploads
        .into_iter()
        .map(|upload| GameVersion {
            id: upload.id.to_string(),
            game_id: game_id.to_string(),
            source: GameSource::Itchio,
            name: upload.display_name.unwrap_or(upload.filename),
            download_size: upload.size.unwrap_or(0),
            external: upload.storage == UploadStorage::External,
        })
        .collect();

    Ok(game_versions)
}

pub async fn fetch_version_info(
    api_key: &str,
    upload_id: &str,
    game: Game,
) -> Result<VersionDownloadInfo> {
    let client = ItchioClient::new(api_key);

    let upload_id: u32 = upload_id.parse()?;
    let game_key: u32 = game.key.clone().unwrap().parse()?;

    let mut retries = 5;

    while retries > 0 {
        let scanned_archive = client
            .fetch_upload_scanned_archive(upload_id, game_key)
            .await;

        if let Ok(scanned_archive) = scanned_archive {
            if scanned_archive.extracted_size.is_some() {
                return Ok(VersionDownloadInfo {
                    install_size: scanned_archive.extracted_size.unwrap(),
                });
            }
        }

        retries -= 1;

        tokio::time::sleep(std::time::Duration::from_secs(2_u64.pow(5 - retries))).await;
    }

    Err("Failed to fetch release info".into())
}

pub async fn pre_download(
    api_key: &str,
    upload_id: &str,
    game: &mut Game,
    download_options: DownloadOptions,
) -> Result<Option<Download>> {
    let client = ItchioClient::new(api_key);

    let upload_id: u32 = upload_id.parse()?;
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

    Ok(Some(Download {
        request: download_request,
        file_name: upload.filename,
        download_options,
        game_source: GameSource::Itchio,
        game_id: game.id.clone(),
        game_title: game.title.clone(),
        md5: upload.md5_hash,
        download_size: upload.size.unwrap_or(0) as u64,
    }))
}

pub async fn post_download(game_id: &str, path: PathBuf, file_name: &str) -> Result<()> {
    let file_path = path.join(file_name);

    let mut connection = database::create_connection()?;
    let mut game = Game::select_one(&mut connection, &GameSource::Itchio, game_id)?;

    if file_path.extension().unwrap() == "zip"
        || file_path.extension().unwrap() == "7z"
        || file_path.extension().unwrap() == "rar"
    {
        println!("Extracting game: {:?}", file_path);
        util::file::extract_file(&file_path, &path).await?;
    }

    let mut launch_target = util::fs::find_launch_target(&path).await?;

    // Strip base path from launch target
    if let Some(target) = &launch_target {
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

    post_download(game_id, path, file_name).await?;

    APP.get()
        .unwrap()
        .emit("download-installed", &payload)
        .unwrap();

    Ok(())
}

pub fn launch_game(game: Game) -> Result<()> {
    let game_path = game.path.unwrap();
    let launch_target = game.launch_target.unwrap();

    let target_path = PathBuf::from(&game_path).join(&launch_target);

    util::file::execute_file(&target_path)?;

    Ok(())
}

pub async fn uninstall_game(game: &Game) -> Result<()> {
    let path = PathBuf::from(game.path.as_ref().unwrap());

    if path.exists() {
        fs::remove_dir_all(&path).await?;
    }

    Ok(())
}

pub async fn handle_external_download(
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
