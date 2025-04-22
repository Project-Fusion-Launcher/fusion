use crate::{common::result::Result, schema::configs::dsl::*};
use diesel::prelude::*;

#[derive(Queryable, Selectable, Insertable, AsChangeset, Clone, Debug, Default)]
#[diesel(table_name = crate::schema::configs)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Config {
    id: i32,
    itchio_api_key: Option<String>,
    legacy_games_token: Option<String>,
    legacy_games_email: Option<String>,
    epic_games_refresh_token: Option<String>,
}

impl Config {
    pub fn select(connection: &mut SqliteConnection) -> Result<Config> {
        let config = configs
            .limit(1)
            .select(Config::as_select())
            .load(connection)?
            .pop()
            .unwrap_or_else(|| Self::new(connection).expect("Failed to create new config"));

        Ok(config)
    }

    pub fn update(&self, connection: &mut SqliteConnection) -> Result<()> {
        diesel::update(configs.filter(id.eq(&self.id)))
            .set(self)
            .execute(connection)?;

        Ok(())
    }

    fn new(connection: &mut SqliteConnection) -> Result<Config> {
        let config = Config::default();

        diesel::insert_into(configs)
            .values(&config)
            .execute(connection)?;

        Ok(config)
    }

    pub fn itchio_api_key(&self) -> Option<String> {
        self.itchio_api_key.clone()
    }

    pub fn set_itchio_api_key(
        &mut self,
        value: Option<String>,
        connection: &mut SqliteConnection,
    ) -> Result<()> {
        self.itchio_api_key = value;
        self.update(connection)?;
        Ok(())
    }

    pub fn legacy_games_email(&self) -> Option<String> {
        self.legacy_games_email.clone()
    }

    pub fn set_legacy_games_email(
        &mut self,
        value: Option<String>,
        connection: &mut SqliteConnection,
    ) -> Result<()> {
        self.legacy_games_email = value;
        self.update(connection)?;
        Ok(())
    }

    pub fn legacy_games_token(&self) -> Option<String> {
        self.legacy_games_token.clone()
    }

    pub fn set_legacy_games_token(
        &mut self,
        value: Option<String>,
        connection: &mut SqliteConnection,
    ) -> Result<()> {
        self.legacy_games_token = value;
        self.update(connection)?;
        Ok(())
    }

    pub fn epic_games_refresh_token(&self) -> Option<String> {
        self.epic_games_refresh_token.clone()
    }

    pub fn set_epic_games_refresh_token(
        &mut self,
        value: Option<String>,
        connection: &mut SqliteConnection,
    ) -> Result<()> {
        self.epic_games_refresh_token = value;
        self.update(connection)?;
        Ok(())
    }
}
