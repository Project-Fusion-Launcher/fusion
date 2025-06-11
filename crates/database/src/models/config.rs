use crate::{
    ConnectionPool,
    schema::configs::{self, dsl::*},
};
use anyhow::Result;
use diesel::prelude::*;
use gpui::Global;
use std::sync::{Arc, RwLock};

#[derive(Clone)]
pub struct Config {
    inner: Arc<RwLock<ConfigModel>>,
    pool: ConnectionPool,
}

impl Config {
    pub fn init(pool: ConnectionPool) -> Result<Self> {
        let config_model = ConfigModel::select(&mut pool.get())?;
        Ok(Self {
            inner: Arc::new(RwLock::new(config_model)),
            pool,
        })
    }

    pub fn it_api_key(&self) -> Option<String> {
        self.inner.read().unwrap().it_api_key.clone()
    }

    pub fn lg_token(&self) -> Option<String> {
        self.inner.read().unwrap().lg_token.clone()
    }

    pub fn lg_email(&self) -> Option<String> {
        self.inner.read().unwrap().lg_email.clone()
    }

    pub fn eg_refresh_token(&self) -> Option<String> {
        self.inner.read().unwrap().eg_refresh_token.clone()
    }

    pub fn set_eg_refresh_token(&mut self, token: Option<String>) -> Result<()> {
        let mut config = self.inner.write().unwrap();
        config.eg_refresh_token = token;
        let mut connection = self.pool.get();
        config.save(&mut connection)?;
        Ok(())
    }
}

impl Global for Config {}

#[derive(Queryable, Identifiable, AsChangeset, Debug)]
#[diesel(table_name = configs)]
#[diesel(check_for_backend(Sqlite))]
#[diesel(treat_none_as_null = true)]
struct ConfigModel {
    id: i32,
    it_api_key: Option<String>,
    lg_token: Option<String>,
    lg_email: Option<String>,
    eg_refresh_token: Option<String>,
}

impl ConfigModel {
    fn select(conn: &mut SqliteConnection) -> Result<ConfigModel> {
        let config = configs.first(conn)?;
        Ok(config)
    }

    fn save(&self, conn: &mut SqliteConnection) -> Result<()> {
        diesel::update(self).set(self).execute(conn)?;

        Ok(())
    }
}
