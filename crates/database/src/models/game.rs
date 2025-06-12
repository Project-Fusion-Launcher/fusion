use crate::{
    ConnectionPool,
    schema::games::{self, dsl::*},
};
use anyhow::Result;
use diesel::prelude::*;
use diesel_derive_enum::DbEnum;
use strum::EnumIter;

#[derive(Queryable, Insertable, Debug)]
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
    pub fn find_one(game_id: &str, game_source: GameSource) -> Result<Game> {
        let game = games
            .find((game_id, game_source))
            .first(&mut ConnectionPool::get_connection())?;
        Ok(game)
    }

    pub fn all() -> Result<Vec<Game>> {
        let all_games = games
            .filter(hidden.eq(false))
            .order(sort_name.asc())
            .load(&mut ConnectionPool::get_connection())?;
        Ok(all_games)
    }

    pub fn insert_or_ignore(values: &[Game]) -> Result<()> {
        diesel::insert_or_ignore_into(games)
            .values(values)
            .execute(&mut ConnectionPool::get_connection())?;

        Ok(())
    }

    pub fn id(&self) -> &str {
        &self.id
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn developer(&self) -> Option<&str> {
        self.developer.as_deref()
    }

    pub fn status(&self) -> GameStatus {
        self.status
    }

    pub fn cover_url(&self) -> Option<&str> {
        self.cover_url.as_deref()
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

#[derive(DbEnum, Clone, Copy, Debug, PartialEq)]
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
    pub name: String,
    pub developer: Option<String>,
    pub cover_url: Option<String>,
}

impl GameBuilder {
    pub fn new(game_id: String, game_source: GameSource, game_name: String) -> Self {
        Self {
            id: game_id,
            source: game_source,
            name: game_name,
            developer: None,
            cover_url: None,
        }
    }

    pub fn developer(mut self, game_developer: String) -> Self {
        self.developer = Some(game_developer);
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
            sort_name: self.name.to_lowercase(),
            name: self.name,
            developer: self.developer,
            status: GameStatus::NotInstalled,
            favorite: false,
            hidden: false,
            cover_url: self.cover_url,
        }
    }
}
