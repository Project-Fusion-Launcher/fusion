use crate::schema::games::dsl::*;
use diesel::prelude::*;
use serde::Serialize;

#[derive(Queryable, Selectable, Insertable, AsChangeset, Clone, Debug, Serialize)]
#[diesel(table_name = crate::schema::games)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Game {
    pub id: String,
    pub source: String,
    pub title: String,
    pub key: Option<String>,
    pub developer: Option<String>,
    pub launch_target: Option<String>,
    pub path: Option<String>,
}

impl Game {
    pub fn select(connection: &mut SqliteConnection, game_source: &str, game_id: &str) -> Game {
        games
            .filter(source.eq(game_source))
            .filter(id.eq(game_id))
            .first(connection)
            .expect("Error loading game")
    }

    pub fn update(&self, connection: &mut SqliteConnection) -> Result<(), diesel::result::Error> {
        diesel::update(games).set(self).execute(connection)?;
        Ok(())
    }
}

/// This is a reduced version of the Game model used to avoid sending unnecessary data to the frontend.
#[derive(Queryable, Selectable, Clone, Debug, Serialize)]
#[diesel(table_name = crate::schema::games)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct ReducedGame {
    pub id: String,
    pub source: String,
    pub title: String,
    pub developer: Option<String>,
}

#[derive(Serialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct GameVersion {
    pub id: String,
    pub game_id: String,
    pub source: String,
    pub name: String,
    pub download_size: u32,
}

#[derive(Serialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct VersionDownloadInfo {
    pub install_size: u32,
}
