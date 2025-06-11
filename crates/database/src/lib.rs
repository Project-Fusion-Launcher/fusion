use anyhow::Result;
use diesel::{
    SqliteConnection,
    r2d2::{ConnectionManager, Pool, PooledConnection},
};
use diesel_migrations::*;
use gpui::Global;
use std::{path::Path, sync::OnceLock};

pub mod models;
mod schema;

const MIGRATIONS: EmbeddedMigrations = embed_migrations!("../../migrations");
static POOL: OnceLock<Pool<ConnectionManager<SqliteConnection>>> = OnceLock::new();

#[derive(Clone)]
pub struct ConnectionPool {
    pool: Pool<ConnectionManager<SqliteConnection>>,
}

impl ConnectionPool {
    pub fn new<P: AsRef<Path>>(path: P) -> Result<Self> {
        let path = path.as_ref().display().to_string();

        let uri = format!("sqlite://{path}?mode=rwc");
        let manager = ConnectionManager::<SqliteConnection>::new(uri);

        let pool = Pool::builder().max_size(5).build(manager)?;

        POOL.set(pool.clone()).unwrap();

        Ok(Self { pool })
    }

    pub fn run_pending_migrations(&self) -> Result<()> {
        let mut connection = self.pool.get()?;
        connection
            .run_pending_migrations(MIGRATIONS)
            .expect("Failed to run database migrations");

        Ok(())
    }

    pub fn get(&self) -> PooledConnection<ConnectionManager<SqliteConnection>> {
        self.pool
            .get()
            .expect("Failed to get a connection from the pool")
    }

    pub fn get_connection() -> PooledConnection<ConnectionManager<SqliteConnection>> {
        POOL.get()
            .expect("Database connection pool not initialized")
            .get()
            .expect("Failed to get a connection from the pool")
    }
}

impl Global for ConnectionPool {}
