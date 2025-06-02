use crate::{common::result::Result, APP};
use diesel::{
    r2d2::{ConnectionManager, Pool, PooledConnection},
    SqliteConnection,
};
use diesel_migrations::*;
use std::fs;
use tauri::Manager;

const MIGRATIONS: EmbeddedMigrations = embed_migrations!();
const DATABASE_NAME: &str = "app_data.db";

pub struct DatabaseManager {
    pool: Pool<ConnectionManager<SqliteConnection>>,
}

impl DatabaseManager {
    pub fn init() -> Result<Self> {
        let app_data_path = APP.get().unwrap().path().app_data_dir()?;
        fs::create_dir_all(&app_data_path)?;
        let database_path = app_data_path.join(DATABASE_NAME);

        let uri = format!("sqlite://{}?mode=rwc", database_path.to_str().unwrap());
        let manager = ConnectionManager::<SqliteConnection>::new(uri);

        let pool = Pool::builder()
            .max_size(5)
            .build(manager)
            .expect("Failed to create database pool");

        let mut connection = pool.get().expect("Failed to get database connection");
        connection
            .run_pending_migrations(MIGRATIONS)
            .expect("Failed to run database migrations");

        Ok(Self { pool })
    }

    pub fn get_connection(&self) -> PooledConnection<ConnectionManager<SqliteConnection>> {
        self.pool.get().expect("Failed to get database connection")
    }

    pub fn connection() -> PooledConnection<ConnectionManager<SqliteConnection>> {
        let manager = APP.get().unwrap().state::<DatabaseManager>();
        manager.get_connection()
    }
}
