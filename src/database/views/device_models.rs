use crudkit::{ReadRecord, ReadRelation, Record, Relation};
use serde::Serialize;

#[derive(Relation, ReadRelation, Serialize)]
#[relation(relation_name = "device_models_view", primary_key = "id")]
pub struct DeviceModelsView {
    records: Vec<DeviceModelsViewRecord>,
}

#[derive(Record, ReadRecord, sqlx::FromRow, Serialize, Clone)]
pub struct DeviceModelsViewRecord {
    pub id: i32,
    pub display_name: String,
    pub manufacturer: String,
    pub category: String,
}
