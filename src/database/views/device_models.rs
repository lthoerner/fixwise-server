use serde::Serialize;

use proc_macros::Relation;

#[derive(Relation, Serialize)]
#[relation(relation_name = "device_models_view", primary_key = "id")]
pub struct DeviceModelsView {
    records: Vec<DeviceModelsViewRecord>,
}

#[derive(sqlx::FromRow, Serialize, Clone)]
pub struct DeviceModelsViewRecord {
    pub id: i32,
    pub display_name: String,
    pub manufacturer: String,
    pub category: String,
}
