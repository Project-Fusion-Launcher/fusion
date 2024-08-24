use crate::{
    managers::{config::ConfigManager, database::DatabaseManager},
    schema::games::dsl::games,
};
use diesel::RunQueryDsl;
use tauri::State;
pub mod itchio;

#[tauri::command]
pub async fn fetch_games(
    config_manager: State<'_, ConfigManager>,
    database_manager: State<'_, DatabaseManager>,
) -> Result<(), String> {
    let mut connection = database_manager.create_connection();

    let itchio_api_key = config_manager.itchio_api_key();
    if let Some(itchio_api_key) = itchio_api_key {
        let itchio_games = itchio::fetch_games(itchio_api_key).await;

        diesel::insert_or_ignore_into(games)
            .values(&itchio_games)
            .execute(&mut connection)
            .unwrap();
    }

    Ok(())
}
