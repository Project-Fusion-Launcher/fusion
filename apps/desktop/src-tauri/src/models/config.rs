use diesel::prelude::*;

#[derive(Queryable, Selectable, Insertable, AsChangeset, Clone, Debug)]
#[diesel(table_name = crate::schema::configs)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Config {
    pub id: i32,
    pub itchio_api_key: Option<String>,
}
