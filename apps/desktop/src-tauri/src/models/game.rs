use super::payloads::GameFilters;
use crate::{
    common::result::Result, managers::database::DatabaseManager, models::events::*,
    schema::games::dsl::*, APP,
};
use diesel::prelude::*;
use diesel_derive_enum::DbEnum;
use serde::{Deserialize, Serialize};
use specta::Type;
use std::path::Path;
use strum_macros::EnumIter;
use tauri_specta::Event;

#[derive(Queryable, Selectable, Insertable, AsChangeset, Clone, Debug, Serialize, Identifiable)]
#[diesel(table_name = crate::schema::games)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
#[diesel(treat_none_as_null = true)]
#[diesel(primary_key(id, source))]
#[serde(rename_all = "camelCase")]
pub struct Game {
    id: String,
    source: GameSource,
    title: String,
    key: Option<String>,
    developer: Option<String>,
    launch_target: Option<String>,
    path: Option<String>,
    version: Option<String>,
    status: GameStatus,
    favorite: bool,
    hidden: bool,
    cover_url: Option<String>,
    sort_title: String,
}

impl Game {
    pub fn find_one(game_id: &str, game_source: GameSource) -> Result<Game> {
        let mut connection = DatabaseManager::connection();
        let game = games
            .filter(source.eq(game_source))
            .filter(id.eq(game_id))
            .first(&mut connection)?;

        Ok(game)
    }

    pub fn insert_or_ignore(values: &[Game]) -> Result<()> {
        let mut connection = DatabaseManager::connection();
        diesel::insert_or_ignore_into(games)
            .values(values)
            .execute(&mut connection)?;

        Ok(())
    }

    pub fn _refresh(&mut self) -> Result<()> {
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

            game.set_status(GameStatus::NotInstalled)?;
        }

        Ok(())
    }

    fn update_with<T>(&self, changeset: T) -> Result<()>
    where
        T: diesel::AsChangeset<Target = games>,
        T::Changeset: diesel::query_builder::QueryFragment<diesel::sqlite::Sqlite>,
    {
        let mut connection = DatabaseManager::connection();
        diesel::update(&self)
            .set(changeset)
            .execute(&mut connection)?;
        Ok(())
    }

    pub fn id(&self) -> &String {
        &self.id
    }

    pub fn source(&self) -> GameSource {
        self.source
    }

    pub fn title(&self) -> &String {
        &self.title
    }

    pub fn key(&self) -> Option<&String> {
        self.key.as_ref()
    }

    pub fn launch_target(&self) -> Option<&String> {
        self.launch_target.as_ref()
    }

    pub fn set_launch_target(&mut self, new_target: Option<String>) -> Result<()> {
        self.launch_target = new_target;
        self.update_with(launch_target.eq(&self.launch_target))
    }

    pub fn path(&self) -> Option<&String> {
        self.path.as_ref()
    }

    pub fn set_path(&mut self, new_path: Option<String>) -> Result<()> {
        self.path = new_path;
        self.update_with(path.eq(&self.path))
    }

    pub fn status(&self) -> GameStatus {
        self.status
    }

    pub fn set_status(&mut self, new_status: GameStatus) -> Result<()> {
        self.status = new_status;
        self.update_with(status.eq(&self.status))?;

        let app_handle = APP.get().unwrap();
        match new_status {
            GameStatus::NotInstalled => GameUninstalled::from(&*self).emit(app_handle),
            GameStatus::Uninstalling => GameUninstalling::from(&*self).emit(app_handle),
            GameStatus::Installing => GameInstalling::from(&*self).emit(app_handle),
            GameStatus::Installed => GameInstalled::from(&*self).emit(app_handle),
            _ => Ok(()),
        }?;

        Ok(())
    }

    pub fn hidden(&self) -> bool {
        self.hidden
    }

    pub fn set_hidden(&mut self, new_hidden: bool) -> Result<()> {
        self.hidden = new_hidden;
        self.update_with(hidden.eq(self.hidden))?;

        if new_hidden {
            GameHidden::from(&*self).emit(APP.get().unwrap())?;
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

#[derive(Serialize, Clone, Debug, Type, PartialEq)]
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

#[derive(DbEnum, Serialize, Deserialize, Clone, Debug, PartialEq, Type, Copy)]
#[serde(rename_all = "camelCase")]
pub enum GameStatus {
    Installed,
    NotInstalled,
    Downloading,
    Installing,
    Uninstalling,
}

pub struct GameBuilder {
    pub id: String,
    pub source: GameSource,
    pub title: String,
    pub key: Option<String>,
    pub developer: Option<String>,
    pub launch_target: Option<String>,
    pub cover_url: Option<String>,
}

impl GameBuilder {
    pub fn new(game_id: String, game_source: GameSource, game_title: String) -> Self {
        Self {
            id: game_id,
            source: game_source,
            title: game_title,
            key: None,
            developer: None,
            launch_target: None,
            cover_url: None,
        }
    }

    pub fn key(mut self, game_key: String) -> Self {
        self.key = Some(game_key);
        self
    }

    pub fn developer(mut self, game_developer: String) -> Self {
        self.developer = Some(game_developer);
        self
    }

    pub fn launch_target(mut self, target: String) -> Self {
        self.launch_target = Some(target);
        self
    }

    pub fn cover_url(mut self, url: String) -> Self {
        self.cover_url = Some(url);
        self
    }

    pub fn build(self) -> Game {
        Game {
            id: self.id,
            source: self.source,
            sort_title: self.title.to_lowercase(),
            title: self.title,
            key: self.key,
            developer: self.developer,
            launch_target: self.launch_target,
            path: None,
            version: None,
            status: GameStatus::NotInstalled,
            favorite: false,
            hidden: false,
            cover_url: self.cover_url,
        }
    }
}
