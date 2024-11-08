use crate::{common::error::Result, schema::configs::dsl::*};
use diesel::prelude::*;

/// A model representing the application configuration.
#[derive(Queryable, Selectable, Insertable, AsChangeset, Clone, Debug, Default)]
#[diesel(table_name = crate::schema::configs)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Config {
    id: i32,
    itchio_api_key: Option<String>,
}

impl Config {
    /// Selects the configuration from the database.
    pub fn select(connection: &mut SqliteConnection) -> Result<Config> {
        let config = configs
            .limit(1)
            .select(Config::as_select())
            .load(connection)?
            .pop()
            .unwrap_or_else(|| Self::new(connection).expect("Failed to create new config"));

        Ok(config)
    }

    /// Updates the configuration in the database.
    pub fn update(&self, connection: &mut SqliteConnection) -> Result<()> {
        diesel::update(configs.filter(id.eq(&self.id)))
            .set(self)
            .execute(connection)?;
        Ok(())
    }

    /// Creates a new configuration in the database. Not meant to be called directly.
    fn new(connection: &mut SqliteConnection) -> Result<Config> {
        let config = Config::default();

        diesel::insert_into(configs)
            .values(&config)
            .execute(connection)?;

        Ok(config)
    }

    // Getters and setters

    pub fn itchio_api_key(&self) -> Option<String> {
        self.itchio_api_key.clone()
    }

    pub fn set_itchio_api_key(&mut self, value: Option<String>) {
        self.itchio_api_key = value;
    }

    pub fn set_itchio_api_key_and_update(
        &mut self,
        connection: &mut SqliteConnection,
        value: Option<String>,
    ) -> Result<()> {
        self.set_itchio_api_key(value);
        self.update(connection)
    }
}
