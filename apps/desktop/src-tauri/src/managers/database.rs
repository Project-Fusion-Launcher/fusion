use crate::{models::config::Config, schema::configs::dsl::*, APP};
use diesel::{
    query_dsl::methods::{LimitDsl, SelectDsl},
    Connection, RunQueryDsl, SelectableHelper, SqliteConnection,
};
use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};
use std::{fs, sync::Mutex};
use tauri::Manager;

pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!();

pub struct DatabaseManager {
    connection: Mutex<SqliteConnection>,
}

impl DatabaseManager {
    pub fn new() -> Self {
        let app_data_path = APP.get().unwrap().path().app_data_dir().unwrap();
        fs::create_dir_all(&app_data_path).expect("Error creating app data directory");
        let database_path = app_data_path.join("data.db");
        let uri = format!("sqlite://{}?mode=rwc", database_path.to_str().unwrap());

        let mut connection =
            SqliteConnection::establish(&uri).expect("Error connecting to database");

        connection.run_pending_migrations(MIGRATIONS).unwrap();

        // TEMPORARY
        let config = configs
            .limit(1)
            .select(Config::as_select())
            .load(&mut connection);

        println!("{:?}", config);

        Self {
            connection: Mutex::new(connection),
        }
    }
}

impl Default for DatabaseManager {
    fn default() -> Self {
        Self::new()
    }
}
