use proc_macros::Relation;

#[derive(Relation)]
#[relation(
    relation_name = "device_models_view",
    primary_key = "id",
    foreign_key_name = "PLACEHOLDER"
)]
pub struct DeviceModelsView {
    records: Vec<DeviceModelsViewRecord>,
}

#[derive(sqlx::FromRow, Clone)]
pub struct DeviceModelsViewRecord {
    pub id: i32,
    pub display_name: String,
    pub manufacturer: String,
    pub category: String,
}
