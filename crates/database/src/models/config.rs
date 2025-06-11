use crate::schema::configs::{self, dsl::*};
use anyhow::Result;
use diesel::prelude::*;
use gpui::Global;
use std::sync::{Arc, RwLock};

#[derive(Clone)]
pub struct Config {
    inner: Arc<RwLock<ConfigModel>>,
}

impl Config {
    pub fn init(conn: &mut SqliteConnection) -> Result<Self> {
        let config_model = ConfigModel::select(conn)?;
        Ok(Self {
            inner: Arc::new(RwLock::new(config_model)),
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
    pub fn select(conn: &mut SqliteConnection) -> Result<ConfigModel> {
        let config = configs.first(conn)?;
        Ok(config)
    }
}
