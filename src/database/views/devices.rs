use crudkit::prelude::*;
use serde::Serialize;

#[derive(Relation, ReadRelation, Serialize)]
#[relation(relation_name = "devices_view", primary_key = "id")]
pub struct DevicesView {
    records: Vec<DevicesViewRecord>,
}

#[derive(Record, ReadRecord, sqlx::FromRow, Serialize, Clone)]
pub struct DevicesViewRecord {
    pub id: i32,
    pub model: String,
    pub owner: Option<String>,
}
