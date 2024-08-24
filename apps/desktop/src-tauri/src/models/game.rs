use diesel::prelude::*;

#[derive(Queryable, Selectable, Insertable, AsChangeset, Clone, Debug)]
#[diesel(table_name = crate::schema::games)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Game {
    pub id: String,
    pub title: String,
}
