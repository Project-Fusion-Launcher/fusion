use crate::{
    common::database,
    managers::download::{DownloadManager, DownloadOptions},
    models::{
        game::{Game, GameSource, GameStatus, GameVersion, GameVersionInfo, ReducedGame},
        payloads::GameFiltersPayload,
    },
    storefronts::get_storefront,
};
use strum::IntoEnumIterator;
use tauri::{AppHandle, Emitter, State};
use tokio::task::JoinSet;

#[tauri::command]
pub async fn get_games(
    refetch: bool,
    filters: Option<GameFiltersPayload>,
) -> Result<Vec<ReducedGame>, String> {
    let mut connection: diesel::SqliteConnection = database::create_connection()?;

    if refetch {
        let mut tasks = JoinSet::new();
        let mut games_to_return = Vec::new();

        for source in GameSource::iter() {
            let store = get_storefront(&source);
            tasks.spawn(async move { store.read().await.fetch_games().await });
        }

        while let Some(res) = tasks.join_next().await {
            match res {
                Ok(fetched_games) => match fetched_games {
                    Ok(Some(fetched_games)) => games_to_return.extend(fetched_games),
                    Ok(None) => (),
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
    game_id: String,
    game_source: GameSource,
) -> Result<Vec<GameVersion>, String> {
    let mut connection = database::create_connection().map_err(|e| e.to_string())?;
    let game =
        Game::select_one(&mut connection, &game_source, &game_id).map_err(|e| e.to_string())?;

    get_storefront(&game_source)
        .read()
        .await
        .fetch_game_versions(game)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn fetch_game_version_info(
    game_id: String,
    game_source: GameSource,
    version_id: String,
) -> Result<GameVersionInfo, String> {
    let mut connection = database::create_connection()?;
    let game = Game::select_one(&mut connection, &game_source, &game_id)?;

    get_storefront(&game_source)
        .read()
        .await
        .fetch_game_version_info(game, version_id)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn download_game(
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

    game.status = GameStatus::Downloading;
    game.path = Some(complete_install_location.to_string_lossy().to_string());
    download_options.install_location = complete_install_location;

    let download = get_storefront(&game_source)
        .read()
        .await
        .pre_download(&mut game, version_id, download_options)
        .await
        .map_err(|e| e.to_string());

    match download {
        Ok(Some(download)) => {
            game.update(&mut connection).unwrap();
            download_manager.enqueue_download(download);
        }
        Ok(None) => {
            game.status = GameStatus::NotInstalled;
            game.update(&mut connection).unwrap();
        }
        Err(e) => return Err(e),
    }

    Ok(())
}

#[tauri::command]
pub async fn launch_game(game_id: String, game_source: GameSource) -> Result<(), String> {
    let mut connection = database::create_connection()?;
    let game = Game::select_one(&mut connection, &game_source, &game_id)?;

    get_storefront(&game_source)
        .read()
        .await
        .launch_game(game)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn uninstall_game(
    app: AppHandle,
    game_id: String,
    game_source: GameSource,
) -> Result<(), String> {
    let mut connection = database::create_connection()?;
    let mut game = Game::select_one(&mut connection, &game_source, &game_id)?;

    game.status = GameStatus::Uninstalling;
    game.update(&mut connection)?;

    app.emit("game-uninstalling", &game)
        .map_err(|e| e.to_string())?;

    get_storefront(&game_source)
        .read()
        .await
        .uninstall_game(&game)
        .await?;

    game.path = None;
    game.status = GameStatus::NotInstalled;
    game.update(&mut connection)?;

    app.emit("game-uninstalled", &game)
        .map_err(|e| e.to_string())?;

    Ok(())
}

#[tauri::command]
pub async fn hide_game(
    app: AppHandle,
    game_id: String,
    game_source: GameSource,
) -> Result<(), String> {
    let mut connection = database::create_connection()?;
    let mut game = Game::select_one(&mut connection, &game_source, &game_id)?;

    game.hidden = true;
    game.update(&mut connection)?;

    app.emit("game-hidden", &game).map_err(|e| e.to_string())?;

    Ok(())
}
