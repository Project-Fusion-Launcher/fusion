use diesel::{
    query_dsl::methods::{LimitDsl, SelectDsl},
    Connection, RunQueryDsl, SelectableHelper, SqliteConnection,
};
use std::{fs, sync::Mutex};
use tauri::Manager;

use crate::{models::config::Config, schemas::config::configs::dsl::*, APP};

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
