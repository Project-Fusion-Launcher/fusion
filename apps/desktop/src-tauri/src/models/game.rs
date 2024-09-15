use diesel::prelude::*;
use serde::Serialize;

#[derive(Queryable, Selectable, Insertable, AsChangeset, Clone, Debug, Serialize)]
#[diesel(table_name = crate::schema::games)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Game {
    pub id: String,
    pub source: String,
    pub title: String,
    pub key: Option<String>,
    pub developer: Option<String>,
}

/// This is a reduced version of the Game model used to avoid sending unnecessary data to the frontend.
#[derive(Queryable, Selectable, Clone, Debug, Serialize)]
#[diesel(table_name = crate::schema::games)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct ReducedGame {
    pub id: String,
    pub source: String,
    pub title: String,
    pub developer: Option<String>,
}
