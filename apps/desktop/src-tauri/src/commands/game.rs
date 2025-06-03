use std::sync::atomic::AtomicU64;

use crate::{
    managers::download::DownloadManager,
    models::{
        download::Download,
        events::{GameHidden, GameUninstalled, GameUninstalling},
        game::{Game, GameSource, GameStatus, GameVersion, GameVersionInfo, ReducedGame},
        payloads::{DownloadOptions, GameFilters},
    },
    storefronts::get_storefront,
};
use strum::IntoEnumIterator;
use tauri::{AppHandle, State};
use tauri_specta::Event;
use tokio::task::JoinSet;

#[tauri::command]
#[specta::specta]
pub async fn get_games(
    refetch: bool,
    filters: Option<GameFilters>,
) -> Result<Vec<ReducedGame>, String> {
    if refetch {
        let mut tasks = JoinSet::new();
        let mut games_to_return = Vec::new();

        for source in GameSource::iter() {
            let store = get_storefront(source);
            tasks.spawn(async move { store.read().await.fetch_games().await });
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

        Game::insert_or_ignore(&games_to_return)?;
    }

    Game::refresh_installed()?;

    let results = ReducedGame::select(filters)?;
    Ok(results)
}

#[tauri::command]
#[specta::specta]
pub async fn fetch_game_versions(
    game_id: String,
    game_source: GameSource,
) -> Result<Vec<GameVersion>, String> {
    let game = Game::select_one(&game_id, game_source).map_err(|e| e.to_string())?;

    get_storefront(game_source)
        .read()
        .await
        .fetch_game_versions(&game)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
#[specta::specta]
pub async fn fetch_game_version_info(
    game_id: String,
    game_source: GameSource,
    version_id: String,
) -> Result<GameVersionInfo, String> {
    let game = Game::select_one(&game_id, game_source)?;

    get_storefront(game_source)
        .read()
        .await
        .fetch_game_version_info(&game, version_id)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
#[specta::specta]
pub async fn download_game(
    download_manager: State<'_, DownloadManager>,
    game_id: String,
    game_source: GameSource,
    version_id: String,
    download_options: DownloadOptions,
) -> Result<(), String> {
    let mut game = Game::select_one(&game_id, game_source)?;

    let complete_install_location = download_options
        .install_location
        .join(game.title.replace(" :", " -").replace(":", " -"));

    game.path = Some(complete_install_location.to_string_lossy().to_string());
    game.status = GameStatus::Downloading;
    game.update()?;

    let version_info = get_storefront(game_source)
        .read()
        .await
        .fetch_game_version_info(&game, version_id.clone())
        .await
        .map_err(|e| e.to_string())?;

    download_manager
        .enqueue(Download {
            game,
            game_version_id: version_id,
            path: complete_install_location,
            download_size: version_info.download_size,
            install_size: version_info.install_size,
            downloaded: AtomicU64::new(0),
            written: AtomicU64::new(0),
        })
        .await?;

    Ok(())
}

#[tauri::command]
#[specta::specta]
pub async fn launch_game(game_id: String, game_source: GameSource) -> Result<(), String> {
    let game = Game::select_one(&game_id, game_source)?;

    get_storefront(game_source)
        .read()
        .await
        .launch_game(game)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
#[specta::specta]
pub async fn uninstall_game(
    app: AppHandle,
    game_id: String,
    game_source: GameSource,
) -> Result<(), String> {
    let mut game = Game::select_one(&game_id, game_source)?;

    game.update_status(GameStatus::Uninstalling)?;
    GameUninstalling::from(&game).emit(&app).unwrap();

    get_storefront(game_source)
        .read()
        .await
        .uninstall_game(&game)
        .await?;

    game.path = None;
    game.status = GameStatus::NotInstalled;
    game.update()?;

    GameUninstalled::from(&game).emit(&app).unwrap();

    Ok(())
}

#[tauri::command]
#[specta::specta]
pub async fn hide_game(
    app: AppHandle,
    game_id: String,
    game_source: GameSource,
) -> Result<(), String> {
    let mut game = Game::select_one(&game_id, game_source)?;

    game.hidden = true;
    game.update()?;

    GameHidden::from(&game).emit(&app).unwrap();

    Ok(())
}
