use crate::{
    managers::{config::ConfigManager, database::DatabaseManager},
    models::game::Game,
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
) -> Result<Vec<Game>, String> {
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

    let results = games
        .select(Game::as_select())
        .load(&mut connection)
        .unwrap();

    Ok(results)
}
