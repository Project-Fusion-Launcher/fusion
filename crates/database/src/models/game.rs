use crate::schema::games::dsl::*;
use anyhow::Result;
use diesel::prelude::*;
use diesel_derive_enum::DbEnum;
use strum::EnumIter;

#[derive(Queryable, Debug)]
#[diesel(table_name = games)]
#[diesel(check_for_backend(Sqlite))]
#[diesel(primary_key(id, source))]
#[diesel(treat_none_as_null = true)]
pub struct Game {
    id: String,
    source: GameSource,
    name: String,
    sort_name: String,
    developer: Option<String>,
    status: GameStatus,
    favorite: bool,
    hidden: bool,
    cover_url: Option<String>,
}

impl Game {
    pub fn find_one(
        conn: &mut SqliteConnection,
        game_id: &str,
        game_source: GameSource,
    ) -> Result<Game> {
        let game = games.find((game_id, game_source)).first(conn)?;
        Ok(game)
    }
}

#[derive(DbEnum, Clone, Copy, Debug, PartialEq, Eq, Hash, EnumIter)]
pub enum GameSource {
    #[db_rename = "eg"]
    EpicGames,
    #[db_rename = "it"]
    Itchio,
    #[db_rename = "lg"]
    LegacyGames,
}

#[derive(DbEnum, Clone, Copy, Debug)]
pub enum GameStatus {
    Installed,
    NotInstalled,
    Downloading,
    Installing,
    Uninstalling,
}
