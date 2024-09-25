use proc_macros::DatabaseEntity;

#[derive(DatabaseEntity)]
#[entity(
    entity_name = "device_models_view",
    primary_key = "id",
    foreign_key_name = "PLACEHOLDER"
)]
pub struct DeviceModelsDatabaseView {
    rows: Vec<DeviceModelsDatabaseViewRow>,
}

#[derive(sqlx::FromRow, Clone)]
pub struct DeviceModelsDatabaseViewRow {
    pub id: i32,
    pub display_name: String,
    pub manufacturer: String,
    pub category: String,
}
