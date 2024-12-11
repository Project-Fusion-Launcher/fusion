use crate::{
    common::{database, result::Result},
    managers::download::{Download, DownloadOptions},
    models::game::{Game, GameSource, GameStatus, GameVersion, VersionDownloadInfo},
    util,
};
use reqwest::header::ETAG;
use std::{path::PathBuf, sync::Arc};
use tokio::{fs, task::JoinSet};
use wrapper_legacygames::{api::models::Product, LegacyGamesClient};

pub async fn fetch_games(email: String, token: Option<String>) -> Result<Vec<Game>> {
    let client = match token {
        Some(token) => Arc::new(LegacyGamesClient::from_token(email, token)),
        None => Arc::new(LegacyGamesClient::from_email(email)),
    };

    let mut join_set = JoinSet::new();

    if client.is_token_client() {
        let client_clone = client.clone();

        join_set.spawn(async move {
            match client_clone.fetch_wp_products().await {
                Ok(products) => {
                    let games = create_games(products, false);
                    Ok(games)
                }
                Err(err) => Err(err),
            }
        });
    }

    join_set.spawn(async move {
        match client.fetch_giveaway_products().await {
            Ok(products) => {
                let games = create_games(products, true);
                Ok(games)
            }
            Err(err) => Err(err),
        }
    });

    let mut result = Vec::new();

    while let Some(res) = join_set.join_next().await {
        match res {
            Ok(games) => result.extend(games?),
            Err(e) => return Err(e.into()),
        }
    }

    Ok(result)
}

pub async fn fetch_game_versions(
    email: String,
    token: Option<String>,
    game: Game,
) -> Result<Vec<GameVersion>> {
    let client = match token {
        Some(token) => LegacyGamesClient::from_token(email, token),
        None => LegacyGamesClient::from_email(email),
    };

    let size = match game.key {
        Some(ref key) => {
            client
                .fetch_wp_installer_size(key.parse()?, &game.id)
                .await?
        }
        None => client.fetch_giveaway_installer_size(&game.id).await?,
    };

    Ok(vec![GameVersion {
        id: game.id.clone(),
        game_id: game.id,
        source: GameSource::LegacyGames,
        name: game.title,
        download_size: size,
    }])
}

pub fn fetch_release_info() -> VersionDownloadInfo {
    // There is no way to fetch the installed size that I know.
    // The game_installed_size in the API's resonse is actually the download size.
    VersionDownloadInfo { install_size: 0 }
}

pub async fn pre_download(
    email: String,
    token: Option<String>,
    game: &mut Game,
    download_options: DownloadOptions,
) -> Result<Download> {
    let client = match token {
        Some(token) => LegacyGamesClient::from_token(email, token),
        None => LegacyGamesClient::from_email(email),
    };

    let installer_url = match game.key {
        Some(ref key) => client.fetch_wp_installer(key.parse()?, &game.id).await?,
        None => client.fetch_giveaway_installer(&game.id).await?,
    };

    let http = reqwest::Client::new();

    // Extract the MD5 hash from the ETag header
    let response = http.head(&installer_url).send().await?;
    let md5 = response
        .headers()
        .get(ETAG)
        .map(|header| header.to_str().unwrap().trim_matches('"').to_string());

    game.version = Some(game.id.clone());

    Ok(Download {
        request: http.get(installer_url),
        file_name: String::from("setup.exe"),
        download_options,
        game_source: GameSource::LegacyGames,
        game_id: game.id.clone(),
        md5,
    })
}

pub async fn post_download(game_id: &str, path: PathBuf, file_name: &str) -> Result<()> {
    let file_path = path.join(file_name);

    let mut connection = database::create_connection()?;
    let mut game = Game::select_one(&mut connection, &GameSource::LegacyGames, game_id)?;

    println!("Extracting game: {:?}", file_path);
    util::file::extract_file(&file_path, &path).await?;

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

    util::file::execute_file(&target_path)?;

    Ok(())
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
                    sort_title: Some(game.game_name.to_lowercase()),
                }
            })
        })
        .collect()
}

pub async fn uninstall_game(game: &Game) -> Result<()> {
    let path = PathBuf::from(game.path.as_ref().unwrap());

    // The uninstaller requires admin. Removing the directory should be enough
    // as nothing is created in the registry (we also don't use the installer).
    if path.exists() {
        fs::remove_dir_all(&path).await?;
    }

    Ok(())
}
