use rust_decimal::Decimal;

use proc_macros::Relation;

#[derive(Relation)]
#[relation(
    relation_name = "products_view",
    primary_key = "sku",
    foreign_key_name = "PLACEHOLDER"
)]
pub struct ProductsView {
    records: Vec<ProductsViewRecord>,
}

#[derive(sqlx::FromRow, Clone)]
pub struct ProductsViewRecord {
    pub sku: i32,
    pub display_name: String,
    pub cost: Decimal,
    pub price: Decimal,
}
