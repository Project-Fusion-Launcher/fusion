use crate::{
    common::{database, error::Result},
    managers::download::{Download, DownloadOptions},
    models::game::{Game, GameSource, GameStatus, GameVersion, VersionDownloadInfo},
    util,
};
use std::{path::PathBuf, process::Stdio};
use tokio::fs;
use wrapper_itchio::ItchioClient;

const NO_WINDOW_FLAGS: u32 = 0x08000000;

pub async fn fetch_games(api_key: &str) -> Result<Vec<Game>> {
    let client = ItchioClient::new(api_key);

    let owned_keys = client.fetch_owned_keys(1).await?;

    let mut games = Vec::new();
    for key in owned_keys.owned_keys {
        let developer = key
            .game
            .user
            .and_then(|user| user.display_name.or(Some(user.username)));

        games.push(Game {
            id: key.game.id.to_string(),
            title: key.game.title,
            source: GameSource::Itchio,
            key: Some(key.id.to_string()),
            developer,
            launch_target: None,
            path: None,
            version: None,
            status: GameStatus::NotInstalled,
        });
    }

    Ok(games)
}

pub async fn fetch_releases(
    api_key: &str,
    game_id: &str,
    game_key: &str,
) -> Result<Vec<GameVersion>> {
    let client = ItchioClient::new(api_key);

    let game_id: u32 = game_id.parse()?;
    let game_key: u32 = game_key.parse()?;
    let game = client.fetch_game_uploads(game_id, game_key).await?;

    let uploads = game
        .uploads
        .into_iter()
        .map(|upload| GameVersion {
            id: upload.id.to_string(),
            game_id: game_id.to_string(),
            source: GameSource::Itchio,
            name: upload.display_name.unwrap_or(upload.filename),
            download_size: upload.size.unwrap_or(0),
        })
        .collect();

    Ok(uploads)
}

pub async fn fetch_release_info(
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

    unimplemented!()
}

pub async fn fetch_download_info(
    api_key: &str,
    upload_id: &str,
    game: &mut Game,
    download_options: DownloadOptions,
) -> Result<Download> {
    let client = ItchioClient::new(api_key);

    let upload_id: u32 = upload_id.parse()?;
    let game_key: u32 = game.key.clone().unwrap().parse()?;

    let upload = client.fetch_game_upload(upload_id, game_key).await?;

    let download_request = client
        .fetch_upload_download_url(upload_id, game_key)
        .await?;

    game.version = upload
        .build
        .as_ref()
        .map(|build| build.version.to_string())
        .or(upload.md5_hash.clone());

    game.path = Some(
        download_options
            .install_location
            .to_string_lossy()
            .into_owned(),
    );

    Ok(Download {
        request: download_request,
        file_name: upload.filename,
        download_options,
        source: GameSource::Itchio,
        game_id: game.id.clone(),
    })
}

pub async fn post_download(game_id: String, path: PathBuf, file_name: String) -> Result<()> {
    let file_path = path.join(file_name);

    let mut connection = database::create_connection()?;
    let mut game = Game::select(&mut connection, &GameSource::Itchio, &game_id)?;

    if file_path.extension().unwrap() == "zip"
        || file_path.extension().unwrap() == "7z"
        || file_path.extension().unwrap() == "rar"
    {
        println!("Extracting archive: {:?}", file_path);
        let exe_path = std::env::current_exe().unwrap();
        let seven_zip = exe_path.parent().unwrap().join("thirdparty/7-Zip/7z.exe");

        let result = tokio::process::Command::new(seven_zip)
            .arg("x")
            .arg(&file_path)
            .arg(format!("-o{}", path.to_string_lossy()))
            .arg("-aoa")
            .creation_flags(NO_WINDOW_FLAGS)
            .output()
            .await;

        println!("{:?}", result);

        fs::remove_file(file_path).await.unwrap();
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

pub fn launch_game(game: Game) -> Result<()> {
    let game_path = game.path.unwrap();
    let launch_target = game.launch_target.unwrap();

    let target_path = PathBuf::from(&game_path).join(&launch_target);

    let result = tokio::process::Command::new(&target_path)
        .current_dir(target_path.parent().unwrap())
        .stdin(Stdio::null())
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .spawn()
        .map_err(|e| e.to_string())?;

    println!("{:?}", result);

    Ok(())
}
