use crate::{
    common::database,
    managers::download::{DownloadManager, DownloadOptions},
    models::{
        config::Config,
        game::{Game, GameSource, GameVersion, ReducedGame, VersionDownloadInfo},
    },
    schema::games::dsl::games,
};
use diesel::{QueryDsl, RunQueryDsl, SelectableHelper};
use std::sync::RwLock;
use tauri::State;
pub mod itchio;

#[tauri::command]
pub async fn get_games(
    config: State<'_, RwLock<Config>>,
    refetch: bool,
) -> Result<Vec<ReducedGame>, String> {
    let mut connection = database::create_connection()?;

    if refetch {
        let mut games_to_return = Vec::new();

        let itchio_api_key = config.read().unwrap().itchio_api_key();
        if let Some(itchio_api_key) = itchio_api_key {
            games_to_return.append(&mut itchio::fetch_games(&itchio_api_key).await?);
        }

        Game::insert_or_ignore(&mut connection, &games_to_return)?;
    }

    Game::refresh_installed(&mut connection)?;

    let results: Vec<ReducedGame> = games
        .select(ReducedGame::as_select())
        .load(&mut connection)
        .unwrap();

    Ok(results)
}

#[tauri::command]
pub async fn fetch_game_versions(
    config: State<'_, RwLock<Config>>,
    game_id: String,
    game_source: GameSource,
) -> Result<Vec<GameVersion>, String> {
    let mut connection = database::create_connection()?;

    let game = Game::select(&mut connection, &game_source, &game_id)?;

    if game_source == GameSource::Itchio {
        let itchio_api_key = config.read().unwrap().itchio_api_key();
        if let Some(itchio_api_key) = itchio_api_key {
            return Ok(
                itchio::fetch_releases(&itchio_api_key, &game_id, &game.key.unwrap()).await?,
            );
        }
    }

    unreachable!()
}

#[tauri::command]
pub async fn fetch_version_info(
    config: State<'_, RwLock<Config>>,
    game_id: String,
    game_source: GameSource,
    version_id: String,
) -> Result<VersionDownloadInfo, String> {
    let mut connection = database::create_connection()?;

    let game = Game::select(&mut connection, &game_source, &game_id)?;

    if game_source == GameSource::Itchio {
        let itchio_api_key = config.read().unwrap().itchio_api_key();
        if let Some(itchio_api_key) = itchio_api_key {
            return Ok(itchio::fetch_release_info(&itchio_api_key, &version_id, game).await?);
        }
    }

    unreachable!()
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

    let mut game = Game::select(&mut connection, &game_source, &game_id)?;

    let complete_install_location = download_options
        .install_location
        .join(game.title.replace(" ", ""));

    download_options.install_location = complete_install_location;

    if game_source == GameSource::Itchio {
        let itchio_api_key = config.read().unwrap().itchio_api_key();
        if let Some(itchio_api_key) = itchio_api_key {
            let download = itchio::fetch_download_info(
                &itchio_api_key,
                &version_id,
                &mut game,
                download_options,
            )
            .await?;

            game.update(&mut connection).unwrap();

            download_manager.enqueue_download(download);
        }
    }

    Ok(())
}

#[tauri::command]
pub async fn launch_game(game_id: String, game_source: GameSource) -> Result<(), String> {
    let mut connection = database::create_connection()?;

    let game = Game::select(&mut connection, &game_source, &game_id)?;

    match game_source {
        GameSource::Itchio => {
            itchio::launch_game(game)?;
        }
    }

    Ok(())
}
