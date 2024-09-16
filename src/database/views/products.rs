use rust_decimal::Decimal;

use proc_macros::DatabaseEntity;

#[derive(DatabaseEntity)]
#[entity(entity_name = "products_view", primary_key = "sku")]
pub struct ProductsDatabaseView {
    rows: Vec<ProductsDatabaseViewRow>,
}

#[derive(sqlx::FromRow, Clone)]
pub struct ProductsDatabaseViewRow {
    pub sku: i32,
    pub display_name: String,
    pub cost: Decimal,
    pub price: Decimal,
}
