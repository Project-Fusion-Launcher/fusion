use crate::{common::database, models::config::Config, schema::configs::dsl::configs};
use diesel::{QueryDsl, RunQueryDsl, SelectableHelper};
use std::sync::RwLock;

/// A manager for handling the application configuration.
pub struct ConfigManager {
    config: RwLock<Config>,
}

impl ConfigManager {
    pub fn init() -> Self {
        let config = Self::load();

        Self {
            config: RwLock::new(config),
        }
    }

    fn _save(&self) {
        let mut connection = database::create_connection();

        diesel::update(configs)
            .set(self.config.read().unwrap().clone())
            .execute(&mut connection)
            .unwrap();
    }

    fn load() -> Config {
        let mut connection = database::create_connection();

        let config = configs
            .limit(1)
            .select(Config::as_select())
            .load(&mut connection)
            .unwrap();

        if config.is_empty() {
            Self::create_default_config()
        } else {
            config[0].clone()
        }
    }

    fn create_default_config() -> Config {
        let mut connection = database::create_connection();

        let config = Config::default();

        diesel::insert_into(configs)
            .values(&config)
            .execute(&mut connection)
            .unwrap();

        config
    }

    // Getters and setters

    pub fn itchio_api_key(&self) -> Option<String> {
        let config = self.config.read().unwrap();
        config.itchio_api_key.clone()
    }

    pub fn set_itchio_api_key(&mut self, itchio_api_key: Option<String>) {
        let mut config = self.config.write().unwrap();
        config.itchio_api_key = itchio_api_key;
    }
}
