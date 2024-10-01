use serde::Serialize;

use proc_macros::Relation;

#[derive(Relation, Serialize)]
#[relation(relation_name = "vendors_view", primary_key = "id")]
pub struct VendorsView {
    records: Vec<VendorsViewRecord>,
}

#[derive(sqlx::FromRow, Serialize, Clone)]
pub struct VendorsViewRecord {
    pub id: i32,
    pub display_name: String,
}
