use super::result::Result;
use crate::APP;
use diesel::{Connection, SqliteConnection};
use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};
use std::{fs, sync::OnceLock};
use tauri::Manager;

const MIGRATIONS: EmbeddedMigrations = embed_migrations!();
const DATABASE_NAME: &str = "app_data.db";

static URI: OnceLock<String> = OnceLock::new();

pub fn init() -> Result<()> {
    let app_data_path = APP.get().unwrap().path().app_data_dir()?;
    fs::create_dir_all(&app_data_path)?;
    let database_path = app_data_path.join(DATABASE_NAME);

    let uri = format!("sqlite://{}?mode=rwc", database_path.to_str().unwrap());
    let mut connection = SqliteConnection::establish(&uri)?;

    connection.run_pending_migrations(MIGRATIONS)?;

    URI.set(uri)?;
    Ok(())
}

pub fn create_connection() -> Result<SqliteConnection> {
    let connection =
        SqliteConnection::establish(URI.get().expect("Error: URI is empty!").as_str())?;
    Ok(connection)
}
