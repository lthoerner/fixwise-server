use rust_decimal::Decimal;

use proc_macros::Relation;

#[derive(Relation)]
#[relation(
    relation_name = "services_view",
    primary_key = "id",
    foreign_key_name = "PLACEHOLDER"
)]
pub struct ServicesView {
    records: Vec<ServicesViewRecord>,
}

#[derive(sqlx::FromRow, Clone)]
pub struct ServicesViewRecord {
    pub id: i32,
    pub type_name: String,
    pub device_name: String,
    pub base_fee: Decimal,
    pub labor_fee: Decimal,
}
