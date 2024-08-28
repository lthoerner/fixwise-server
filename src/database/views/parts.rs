use rust_decimal::Decimal;

use proc_macros::DatabaseEntity;

#[derive(DatabaseEntity)]
#[entity(entity_name = "parts_view", primary_key = "id")]
pub struct PartsDatabaseView {
    rows: Vec<PartsDatabaseViewRow>,
}

#[derive(sqlx::FromRow, Clone)]
pub struct PartsDatabaseViewRow {
    pub id: i32,
    pub display_name: String,
    pub vendor: String,
    pub manufacturer: Option<String>,
    pub category: String,
    pub cost: Option<Decimal>,
    pub price: Option<Decimal>,
}
