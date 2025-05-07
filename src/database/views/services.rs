use crudkit::{ReadRecord, ReadRelation, Record, Relation};
use rust_decimal::Decimal;
use serde::Serialize;

#[derive(Relation, ReadRelation, Serialize)]
#[relation(relation_name = "services_view", primary_key = "id")]
pub struct ServicesView {
    records: Vec<ServicesViewRecord>,
}

#[derive(Record, ReadRecord, sqlx::FromRow, Serialize, Clone)]
pub struct ServicesViewRecord {
    pub id: i32,
    pub type_name: String,
    pub device_name: String,
    pub base_fee: Decimal,
    pub labor_fee: Decimal,
}
