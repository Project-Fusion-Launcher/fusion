use std::path::Path;

use crate::{common::error::Result, schema::games::dsl::*};
use diesel::prelude::*;
use diesel_derive_enum::DbEnum;
use serde::{Deserialize, Serialize};

#[derive(Queryable, Selectable, Insertable, AsChangeset, Clone, Debug, Serialize)]
#[diesel(table_name = crate::schema::games)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Game {
    pub id: String,
    pub source: GameSource,
    pub title: String,
    pub key: Option<String>,
    pub developer: Option<String>,
    pub launch_target: Option<String>,
    pub path: Option<String>,
    pub version: Option<String>,
    pub status: GameStatus,
}

impl Game {
    pub fn select(
        connection: &mut SqliteConnection,
        game_source: &GameSource,
        game_id: &str,
    ) -> Result<Game> {
        let game = games
            .filter(source.eq(game_source))
            .filter(id.eq(game_id))
            .first(connection)?;

        Ok(game)
    }

    pub fn update(&self, connection: &mut SqliteConnection) -> Result<()> {
        diesel::update(games.filter(id.eq(&self.id)))
            .set(self)
            .execute(connection)?;

        Ok(())
    }

    pub fn insert_or_ignore(connection: &mut SqliteConnection, values: &[Game]) -> Result<()> {
        diesel::insert_or_ignore_into(games)
            .values(values)
            .execute(connection)?;

        Ok(())
    }

    /// Refreshes the status of installed games in case they were manually removed.
    pub fn refresh_installed(connection: &mut SqliteConnection) -> Result<()> {
        let installed_games = games
            .filter(status.eq(GameStatus::Installed))
            .load::<Game>(connection)?;

        for mut game in installed_games {
            if let Some(game_path) = &game.path {
                if Path::new(game_path).exists() {
                    continue;
                }
            }

            game.status = GameStatus::NotInstalled;
            game.update(connection)?;
        }

        Ok(())
    }
}

/// This is a reduced version of the Game model used to avoid sending unnecessary data to the frontend.
#[derive(Queryable, Selectable, Clone, Debug, Serialize)]
#[diesel(table_name = crate::schema::games)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct ReducedGame {
    pub id: String,
    pub source: GameSource,
    pub title: String,
    pub developer: Option<String>,
    pub path: Option<String>,
    pub status: GameStatus,
}

#[derive(Serialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct GameVersion {
    pub id: String,
    pub game_id: String,
    pub source: GameSource,
    pub name: String,
    pub download_size: u32,
}

#[derive(Serialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct VersionDownloadInfo {
    pub install_size: u32,
}

#[derive(DbEnum, Serialize, Deserialize, Clone, Debug, PartialEq)]
#[serde(rename_all = "camelCase")]
pub enum GameSource {
    Itchio,
    LegacyGames,
}

#[derive(DbEnum, Serialize, Deserialize, Clone, Debug, PartialEq)]
#[serde(rename_all = "camelCase")]
pub enum GameStatus {
    Installed,
    NotInstalled,
}
