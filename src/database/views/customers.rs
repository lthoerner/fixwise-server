use sqlx::FromRow;

use crate::database::DatabaseEntity;

#[derive(FromRow)]
pub struct CustomersDatabaseViewRow {
    pub id: i32,
    pub name: String,
    pub email: String,
    pub phone: String,
    pub address: Option<String>,
}

impl DatabaseEntity for CustomersDatabaseViewRow {
    const ENTITY_NAME: &'static str = "customers_view";
    const PRIMARY_COLUMN_NAME: &'static str = "id";
}
