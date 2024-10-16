use crate::{
    managers::{
        config::ConfigManager,
        database::DatabaseManager,
        download::{DownloadManager, DownloadOptions},
    },
    models::game::{Game, GameVersion, ReducedGame, VersionDownloadInfo},
    schema::games::dsl::games,
};
use diesel::{QueryDsl, RunQueryDsl, SelectableHelper};
use tauri::State;
pub mod itchio;

#[tauri::command]
pub async fn get_games(
    config_manager: State<'_, ConfigManager>,
    database_manager: State<'_, DatabaseManager>,
    refetch: bool,
) -> Result<Vec<ReducedGame>, String> {
    let mut connection = database_manager.create_connection();

    if refetch {
        let mut games_to_return = Vec::new();

        let itchio_api_key = config_manager.itchio_api_key();
        if let Some(itchio_api_key) = itchio_api_key {
            games_to_return.append(&mut itchio::fetch_games(itchio_api_key).await);
        }

        diesel::insert_or_ignore_into(games)
            .values(&games_to_return)
            .execute(&mut connection)
            .unwrap();
    }

    let results: Vec<ReducedGame> = games
        .select(ReducedGame::as_select())
        .load(&mut connection)
        .unwrap();

    Ok(results)
}

#[tauri::command]
pub async fn fetch_game_versions(
    config_manager: State<'_, ConfigManager>,
    database_manager: State<'_, DatabaseManager>,
    game_id: String,
    game_source: String,
) -> Result<Vec<GameVersion>, String> {
    let mut connection = database_manager.create_connection();

    let game = Game::select_from_id(&mut connection, &game_source, &game_id);

    if game_source == "itchio" {
        let itchio_api_key = config_manager.itchio_api_key();
        if let Some(itchio_api_key) = itchio_api_key {
            return Ok(itchio::fetch_releases(itchio_api_key, &game_id, &game.key.unwrap()).await);
        }
    }

    unreachable!()
}

#[tauri::command]
pub async fn fetch_version_info(
    config_manager: State<'_, ConfigManager>,
    database_manager: State<'_, DatabaseManager>,
    game_id: String,
    game_source: String,
    version_id: String,
) -> Result<VersionDownloadInfo, String> {
    let mut connection = database_manager.create_connection();

    let game = Game::select_from_id(&mut connection, &game_source, &game_id);

    if game_source == "itchio" {
        let itchio_api_key = config_manager.itchio_api_key();
        if let Some(itchio_api_key) = itchio_api_key {
            return Ok(itchio::fetch_release_info(itchio_api_key, &version_id, game).await);
        }
    }

    unreachable!()
}

#[tauri::command]
pub async fn download_game(
    config_manager: State<'_, ConfigManager>,
    database_manager: State<'_, DatabaseManager>,
    download_manager: State<'_, DownloadManager>,
    game_id: String,
    game_source: String,
    version_id: String,
    download_options: DownloadOptions,
) -> Result<(), String> {
    let mut connection = database_manager.create_connection();

    let game = Game::select_from_id(&mut connection, &game_source, &game_id);

    if game_source == "itchio" {
        let itchio_api_key = config_manager.itchio_api_key();
        if let Some(itchio_api_key) = itchio_api_key {
            let download =
                itchio::fetch_download_info(itchio_api_key, &version_id, game, download_options)
                    .await;

            download_manager.enqueue_download(download);
        }
    }

    Ok(())
}
