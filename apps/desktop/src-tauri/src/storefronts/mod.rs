use crate::{
    common::database,
    managers::download::{DownloadManager, DownloadOptions},
    models::{
        config::Config,
        game::{Game, GameSource, GameStatus, GameVersion, ReducedGame, VersionDownloadInfo},
        payloads::{GameFiltersPayload, GameUninstalledPayload},
    },
};
use std::sync::RwLock;
use tauri::{AppHandle, Emitter, State};
use tokio::task::JoinSet;

pub mod itchio;
pub mod legacygames;

#[tauri::command]
pub async fn get_games(
    config: State<'_, RwLock<Config>>,
    refetch: bool,
    filters: Option<GameFiltersPayload>,
) -> Result<Vec<ReducedGame>, String> {
    let mut connection = database::create_connection()?;

    if refetch {
        let mut tasks = JoinSet::new();
        let mut games_to_return = Vec::new();

        // itch.io
        let itchio_api_key = config.read().unwrap().itchio_api_key();
        if let Some(itchio_api_key) = itchio_api_key {
            tasks.spawn(async move { itchio::fetch_games(&itchio_api_key).await });
        }

        // Legacy Games
        let legacy_games_email = config.read().unwrap().legacy_games_email();
        let legacy_games_token = config.read().unwrap().legacy_games_token();
        if let Some(legacy_games_email) = legacy_games_email {
            tasks.spawn(async move {
                legacygames::fetch_games(legacy_games_email, legacy_games_token).await
            });
        }

        while let Some(res) = tasks.join_next().await {
            match res {
                Ok(fetched_games) => match fetched_games {
                    Ok(fetched_games) => games_to_return.extend(fetched_games),
                    Err(e) => println!("{:?}", e),
                },
                Err(e) => println!("{:?}", e),
            }
        }

        Game::insert_or_ignore(&mut connection, &games_to_return)?;
    }

    Game::refresh_installed(&mut connection)?;

    let results = ReducedGame::select(&mut connection, filters)?;
    Ok(results)
}

#[tauri::command]
pub async fn fetch_game_versions(
    config: State<'_, RwLock<Config>>,
    game_id: String,
    game_source: GameSource,
) -> Result<Vec<GameVersion>, String> {
    let mut connection = database::create_connection().map_err(|e| e.to_string())?;

    let game =
        Game::select_one(&mut connection, &game_source, &game_id).map_err(|e| e.to_string())?;

    match game_source {
        GameSource::Itchio => {
            let itchio_api_key = config.read().unwrap().itchio_api_key();
            if let Some(api_key) = itchio_api_key {
                return itchio::fetch_game_versions(&api_key, &game_id, &game.key.unwrap())
                    .await
                    .map_err(|e| e.to_string());
            }
        }
        GameSource::LegacyGames => {
            let legacy_games_email = config.read().unwrap().legacy_games_email();
            let legacy_games_token = config.read().unwrap().legacy_games_token();
            if let Some(email) = legacy_games_email {
                return legacygames::fetch_game_versions(email, legacy_games_token, game)
                    .await
                    .map_err(|e| e.to_string());
            }
        }
    }

    Err("Invalid game source or missing credentials".to_string())
}

#[tauri::command]
pub async fn fetch_version_info(
    config: State<'_, RwLock<Config>>,
    game_id: String,
    game_source: GameSource,
    version_id: String,
) -> Result<VersionDownloadInfo, String> {
    let mut connection = database::create_connection()?;

    let game = Game::select_one(&mut connection, &game_source, &game_id)?;

    match game_source {
        GameSource::Itchio => {
            let itchio_api_key = config.read().unwrap().itchio_api_key();
            if let Some(api_key) = itchio_api_key {
                return Ok(itchio::fetch_version_info(&api_key, &version_id, game).await?);
            }
        }
        GameSource::LegacyGames => {
            return Ok(legacygames::fetch_version_info());
        }
    }

    Err("Invalid game source or missing credentials".to_string())
}

#[tauri::command]
pub async fn download_game(
    config: State<'_, RwLock<Config>>,
    download_manager: State<'_, DownloadManager>,
    game_id: String,
    game_source: GameSource,
    version_id: String,
    mut download_options: DownloadOptions,
) -> Result<(), String> {
    let mut connection = database::create_connection()?;

    let mut game = Game::select_one(&mut connection, &game_source, &game_id)?;

    let complete_install_location = download_options
        .install_location
        .join(game.title.replace(" :", " -").replace(":", " -"));

    game.path = Some(complete_install_location.to_string_lossy().to_string());
    download_options.install_location = complete_install_location;

    let download = match game_source {
        GameSource::Itchio => {
            let itchio_api_key = config.read().unwrap().itchio_api_key();
            if let Some(api_key) = itchio_api_key {
                itchio::pre_download(&api_key, &version_id, &mut game, download_options).await?
            } else {
                return Err("Missing itch.io API key".to_string());
            }
        }
        GameSource::LegacyGames => {
            let legacy_games_email = config.read().unwrap().legacy_games_email();
            let legacy_games_token = config.read().unwrap().legacy_games_token();
            if let Some(email) = legacy_games_email {
                legacygames::pre_download(email, legacy_games_token, &mut game, download_options)
                    .await?
            } else {
                return Err("Missing Legacy Games credentials".to_string());
            }
        }
    };

    game.update(&mut connection).unwrap();
    download_manager.enqueue_download(download);

    Ok(())
}

#[tauri::command]
pub async fn launch_game(game_id: String, game_source: GameSource) -> Result<(), String> {
    let mut connection = database::create_connection()?;

    let game = Game::select_one(&mut connection, &game_source, &game_id)?;

    match game_source {
        GameSource::Itchio => {
            itchio::launch_game(game)?;
        }
        GameSource::LegacyGames => {
            legacygames::launch_game(game)?;
        }
    }

    Ok(())
}

#[tauri::command]
pub async fn uninstall_game(
    app_handle: AppHandle,
    game_id: String,
    game_source: GameSource,
) -> Result<(), String> {
    let mut connection = database::create_connection()?;

    let mut game = Game::select_one(&mut connection, &game_source, &game_id)?;

    match game_source {
        GameSource::Itchio => {
            itchio::uninstall_game(&game).await?;
        }
        GameSource::LegacyGames => {
            legacygames::uninstall_game(&game).await?;
        }
    }

    game.path = None;
    game.status = GameStatus::NotInstalled;
    game.update(&mut connection)?;

    app_handle
        .emit(
            "game-uninstalled",
            GameUninstalledPayload {
                game_id,
                game_source,
            },
        )
        .map_err(|e| e.to_string())?;

    Ok(())
}

#[tauri::command]
pub async fn hide_game(game_id: String, game_source: GameSource) -> Result<(), String> {
    let mut connection = database::create_connection()?;

    let mut game = Game::select_one(&mut connection, &game_source, &game_id)?;

    game.hidden = true;
    game.update(&mut connection)?;

    Ok(())
}
