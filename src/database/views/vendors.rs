use serde::Serialize;

use proc_macros::{ReadRelation, Relation};

#[derive(Relation, ReadRelation, Serialize)]
#[relation(relation_name = "vendors_view", primary_key = "id")]
pub struct VendorsView {
    records: Vec<VendorsViewRecord>,
}

#[derive(sqlx::FromRow, Serialize, Clone)]
pub struct VendorsViewRecord {
    pub id: i32,
    pub display_name: String,
}
