use proc_macros::DatabaseEntity;

#[derive(DatabaseEntity)]
#[entity(
    entity_name = "vendors_view",
    primary_key = "id",
    foreign_key_name = "PLACEHOLDER"
)]
pub struct VendorsDatabaseView {
    rows: Vec<VendorsDatabaseViewRow>,
}

#[derive(sqlx::FromRow, Clone)]
pub struct VendorsDatabaseViewRow {
    pub id: i32,
    pub display_name: String,
}
