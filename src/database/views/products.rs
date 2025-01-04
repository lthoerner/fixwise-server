use rust_decimal::Decimal;
use serde::Serialize;

use proc_macros::{ReadRelation, Relation};

#[derive(Relation, ReadRelation, Serialize)]
#[relation(relation_name = "products_view", primary_key = "sku")]
pub struct ProductsView {
    records: Vec<ProductsViewRecord>,
}

#[derive(sqlx::FromRow, Serialize, Clone)]
pub struct ProductsViewRecord {
    pub sku: i32,
    pub display_name: String,
    pub cost: Decimal,
    pub price: Decimal,
}
