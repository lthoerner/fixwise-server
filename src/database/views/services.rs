use rust_decimal::Decimal;

use proc_macros::DatabaseEntity;

#[derive(DatabaseEntity)]
#[entity(entity_name = "services_view", primary_key = "id")]
pub struct ServicesDatabaseView {
    rows: Vec<ServicesDatabaseViewRow>,
}

#[derive(sqlx::FromRow, Clone)]
pub struct ServicesDatabaseViewRow {
    pub id: i32,
    pub type_name: String,
    pub device_name: String,
    pub base_fee: Decimal,
    pub labor_fee: Decimal,
}
