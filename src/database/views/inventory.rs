use rust_decimal::Decimal;
use sqlx::FromRow;

use crate::database::DatabaseEntity;

#[derive(FromRow)]
pub struct InventoryDatabaseViewRow {
    pub sku: i32,
    pub name: String,
    pub count: i32,
    pub price: Decimal,
    pub cost: Decimal,
}

impl DatabaseEntity for InventoryDatabaseViewRow {
    const ENTITY_NAME: &'static str = "inventory_view";
    const PRIMARY_COLUMN_NAME: &'static str = "sku";
}
