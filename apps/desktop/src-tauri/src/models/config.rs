use crate::schema::configs::dsl::*;
use diesel::prelude::*;

/// A model representing the application configuration.
#[derive(Queryable, Selectable, Insertable, AsChangeset, Clone, Debug, Default)]
#[diesel(table_name = crate::schema::configs)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Config {
    pub id: i32,
    pub itchio_api_key: Option<String>,
}

impl Config {
    /// Updates the configuration in the database.
    pub fn update(&self, connection: &mut SqliteConnection) -> Result<(), diesel::result::Error> {
        diesel::update(configs.filter(id.eq(&self.id)))
            .set(self)
            .execute(connection)?;
        Ok(())
    }

    /// Selects the configuration from the database.
    pub fn select(connection: &mut SqliteConnection) -> Config {
        configs
            .limit(1)
            .select(Config::as_select())
            .load(connection)
            .expect("Error loading config")
            .pop()
            .unwrap_or_else(|| Self::new(connection))
    }

    /// Creates a new configuration in the database. Not meant to be called directly.
    fn new(connection: &mut SqliteConnection) -> Config {
        let config = Config::default();

        diesel::insert_into(configs)
            .values(&config)
            .execute(connection)
            .expect("Error creating config");

        config
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
    ) -> Result<(), diesel::result::Error> {
        self.set_itchio_api_key(value);
        self.update(connection)
    }
}
