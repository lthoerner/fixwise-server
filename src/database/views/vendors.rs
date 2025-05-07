use crudkit::prelude::*;
use serde::Serialize;

#[derive(Relation, ReadRelation, Serialize)]
#[relation(relation_name = "vendors_view", primary_key = "id")]
pub struct VendorsView {
    records: Vec<VendorsViewRecord>,
}

#[derive(Record, ReadRecord, sqlx::FromRow, Serialize, Clone)]
pub struct VendorsViewRecord {
    pub id: i32,
    pub display_name: String,
}
