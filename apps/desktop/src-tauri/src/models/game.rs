use super::payloads::GameFilters;
use crate::{common::result::Result, managers::database::DatabaseManager, schema::games::dsl::*};
use diesel::prelude::*;
use diesel_derive_enum::DbEnum;
use serde::{Deserialize, Serialize};
use specta::Type;
use std::path::Path;
use strum_macros::EnumIter;

#[derive(Queryable, Selectable, Insertable, AsChangeset, Clone, Debug, Serialize, Identifiable)]
#[diesel(table_name = crate::schema::games)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
#[diesel(treat_none_as_null = true)]
#[diesel(primary_key(id, source))]
#[serde(rename_all = "camelCase")]
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
    pub favorite: bool,
    pub hidden: bool,
    pub cover_url: Option<String>,
    pub sort_title: String,
}

impl Game {
    pub fn select_one(game_id: &str, game_source: GameSource) -> Result<Game> {
        let mut connection = DatabaseManager::connection();
        let game = games
            .filter(source.eq(game_source))
            .filter(id.eq(game_id))
            .first(&mut connection)?;

        Ok(game)
    }

    pub fn update(&self) -> Result<()> {
        let mut connection = DatabaseManager::connection();
        self.save_changes::<Self>(&mut connection)?;

        Ok(())
    }

    pub fn insert_or_ignore(values: &[Game]) -> Result<()> {
        let mut connection = DatabaseManager::connection();
        diesel::insert_or_ignore_into(games)
            .values(values)
            .execute(&mut connection)?;

        Ok(())
    }

    pub fn update_status(&mut self, new_status: GameStatus) -> Result<()> {
        let mut connection = DatabaseManager::connection();
        self.status = new_status;
        diesel::update(&*self)
            .set(status.eq(&self.status))
            .execute(&mut connection)?;

        Ok(())
    }

    pub fn refresh(&mut self) -> Result<()> {
        let mut connection = DatabaseManager::connection();
        let updated_game = games
            .filter(id.eq(&self.id))
            .filter(source.eq(&self.source))
            .first::<Game>(&mut connection)?;

        *self = updated_game;
        Ok(())
    }

    /// Refreshes the status of installed games in case they were manually removed.
    pub fn refresh_installed() -> Result<()> {
        let mut connection = DatabaseManager::connection();
        let installed_games = games
            .filter(status.eq(GameStatus::Installed))
            .load::<Game>(&mut connection)?;

        for mut game in installed_games {
            if let Some(game_path) = &game.path {
                if Path::new(game_path).exists() {
                    continue;
                }
            }

            game.update_status(GameStatus::NotInstalled)?;
        }

        Ok(())
    }
}

/// This is a reduced version of the Game model used to avoid sending unnecessary data to the frontend.
#[derive(Queryable, Selectable, Clone, Debug, Serialize, Type)]
#[diesel(table_name = crate::schema::games)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
#[serde(rename_all = "camelCase")]
#[serde(rename = "Game")]
pub struct ReducedGame {
    pub id: String,
    pub source: GameSource,
    pub title: String,
    pub developer: Option<String>,
    pub path: Option<String>,
    pub status: GameStatus,
    pub favorite: bool,
    pub hidden: bool,
    pub cover_url: Option<String>,
}

impl From<Game> for ReducedGame {
    fn from(game: Game) -> Self {
        ReducedGame {
            id: game.id,
            source: game.source,
            title: game.title,
            developer: game.developer,
            path: game.path,
            status: game.status,
            favorite: game.favorite,
            hidden: game.hidden,
            cover_url: game.cover_url,
        }
    }
}

impl ReducedGame {
    pub fn select(filters: Option<GameFilters>) -> Result<Vec<ReducedGame>> {
        let mut statement = games.select(ReducedGame::as_select()).into_boxed();

        statement = statement.filter(hidden.eq(false)).order(sort_title.asc());

        if let Some(filters) = filters {
            if let Some(query) = filters.query {
                for query in query.split_whitespace() {
                    statement = statement.filter(title.like(format!("%{}%", query)));
                }
            }
        }

        let mut connection = DatabaseManager::connection();
        let results: Vec<ReducedGame> = statement.load(&mut connection).unwrap();

        Ok(results)
    }
}

#[derive(Serialize, Clone, Debug, Type)]
#[serde(rename_all = "camelCase")]
pub struct GameVersion {
    pub id: String,
    pub name: String,
    pub external: bool,
}

#[derive(Serialize, Clone, Debug, Type)]
#[serde(rename_all = "camelCase")]
pub struct GameVersionInfo {
    pub install_size: u64,
    pub download_size: u64,
}

#[derive(
    DbEnum, Serialize, Deserialize, Clone, Debug, PartialEq, EnumIter, Copy, Type, Eq, Hash,
)]
#[serde(rename_all = "camelCase")]
pub enum GameSource {
    Itchio,
    LegacyGames,
    EpicGames,
}

#[derive(DbEnum, Serialize, Deserialize, Clone, Debug, PartialEq, Type)]
#[serde(rename_all = "camelCase")]
pub enum GameStatus {
    Installed,
    NotInstalled,
    Downloading,
    Installing,
    Uninstalling,
}
