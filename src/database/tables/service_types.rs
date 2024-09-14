use proc_macros::{BulkInsert, DatabaseEntity, IdentifiableRow, SingleInsert};

use crate::database::{GenerateStaticRowData, GenerateStaticTableData};

#[derive(DatabaseEntity, BulkInsert, Clone)]
#[entity(entity_name = "service_types", primary_key = "id")]
pub struct ServiceTypesDatabaseTable {
    rows: Vec<ServiceTypesDatabaseTableRow>,
}

#[derive(SingleInsert, sqlx::FromRow, IdentifiableRow, Clone)]
pub struct ServiceTypesDatabaseTableRow {
    pub id: i32,
    pub display_name: String,
}

impl GenerateStaticTableData for ServiceTypesDatabaseTable {
    const ITEMS: &[&str] = &[
        "Screen Repair",
        "Battery Repair",
        "Backglass Repair",
        "Camera Repair",
        "Port Repair",
        "Other Repair",
    ];
}

impl GenerateStaticRowData for ServiceTypesDatabaseTableRow {
    fn new(id: i32, display_name: impl Into<String>) -> Self {
        Self {
            id,
            display_name: display_name.into(),
        }
    }
}
