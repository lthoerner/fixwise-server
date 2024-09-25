use proc_macros::DatabaseEntity;

#[derive(DatabaseEntity)]
#[entity(
    entity_name = "devices_view",
    primary_key = "id",
    foreign_key_name = "PLACEHOLDER"
)]
pub struct DevicesDatabaseView {
    rows: Vec<DevicesDatabaseViewRow>,
}

#[derive(sqlx::FromRow, Clone)]
pub struct DevicesDatabaseViewRow {
    pub id: i32,
    pub model: String,
    pub owner: Option<String>,
}
