use crate::{models::config::Config, schema::configs::dsl::configs, APP};
use diesel::{QueryDsl, RunQueryDsl, SelectableHelper};
use tauri::Manager;

use super::database::DatabaseManager;

pub struct ConfigManager {
    config: Config,
}

impl ConfigManager {
    pub fn new() -> Self {
        let config = Self::load();

        Self { config }
    }

    fn save(&self) {
        let database_manager = APP.get().unwrap().state::<DatabaseManager>();
        let mut connection = database_manager.create_connection();

        diesel::update(configs)
            .set(self.config.clone())
            .execute(&mut connection)
            .unwrap();
    }

    fn load() -> Config {
        let database_manager = APP.get().unwrap().state::<DatabaseManager>();
        let mut connection = database_manager.create_connection();

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
        let database_manager = APP.get().unwrap().state::<DatabaseManager>();
        let mut connection = database_manager.create_connection();

        let config = Config {
            id: 0,
            itchio_api_key: Some("JrfKUqAAjkGeWc4uzpCcE3tIxyofoYDlQBMYabZc".to_string()),
        };

        diesel::insert_into(configs)
            .values(&config)
            .execute(&mut connection)
            .unwrap();

        config
    }

    // Getters and setters

    pub fn itchio_api_key(&self) -> Option<&str> {
        self.config.itchio_api_key.as_deref()
    }

    pub fn set_itchio_api_key(&mut self, itchio_api_key: Option<String>) {
        self.config.itchio_api_key = itchio_api_key;
    }
}

impl Default for ConfigManager {
    fn default() -> Self {
        Self::new()
    }
}
