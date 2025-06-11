use crate::schema::configs::{self, dsl::*};
use anyhow::Result;
use diesel::prelude::*;
use gpui::Global;

#[derive(Queryable, Identifiable, AsChangeset, Debug)]
#[diesel(table_name = configs)]
#[diesel(check_for_backend(Sqlite))]
#[diesel(treat_none_as_null = true)]
pub struct Config {
    id: i32,
    it_api_key: Option<String>,
    lg_token: Option<String>,
    lg_email: Option<String>,
    eg_refresh_token: Option<String>,
}

impl Config {
    pub fn select(conn: &mut SqliteConnection) -> Result<Config> {
        let config = configs.first(conn)?;
        Ok(config)
    }

    pub fn it_api_key(&self) -> Option<&str> {
        self.it_api_key.as_deref()
    }

    pub fn lg_token(&self) -> Option<&str> {
        self.lg_token.as_deref()
    }

    pub fn lg_email(&self) -> Option<&str> {
        self.lg_email.as_deref()
    }

    pub fn eg_refresh_token(&self) -> Option<&str> {
        self.eg_refresh_token.as_deref()
    }
}

impl Global for Config {}
