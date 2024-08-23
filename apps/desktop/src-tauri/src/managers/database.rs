use crate::APP;
use diesel::{Connection, SqliteConnection};
use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};
use std::fs;
use tauri::Manager;

const MIGRATIONS: EmbeddedMigrations = embed_migrations!();

const DATABASE_NAME: &str = "app_data.db";

pub struct DatabaseManager {
    uri: String,
}

impl DatabaseManager {
    pub fn new() -> Self {
        let app_data_path = APP.get().unwrap().path().app_data_dir().unwrap();
        fs::create_dir_all(&app_data_path).expect("Error creating app data directory");
        let database_path = app_data_path.join(DATABASE_NAME);

        let uri = format!("sqlite://{}?mode=rwc", database_path.to_str().unwrap());

        let mut connection =
            SqliteConnection::establish(&uri).expect("Error connecting to database");

        connection.run_pending_migrations(MIGRATIONS).unwrap();

        Self { uri }
    }

    pub fn create_connection(&self) -> SqliteConnection {
        SqliteConnection::establish(&self.uri).expect("Error connecting to database")
    }
}

impl Default for DatabaseManager {
    fn default() -> Self {
        Self::new()
    }
}
