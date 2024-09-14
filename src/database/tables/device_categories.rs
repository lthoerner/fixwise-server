use proc_macros::{BulkInsert, DatabaseEntity, IdentifiableRow, SingleInsert};

use crate::database::{GenerateStaticRowData, GenerateStaticTableData};

#[derive(DatabaseEntity, BulkInsert, Clone)]
#[entity(entity_name = "device_categories", primary_key = "id")]
pub struct DeviceCategoriesDatabaseTable {
    rows: Vec<DeviceCategoriesDatabaseTableRow>,
}

#[derive(SingleInsert, sqlx::FromRow, IdentifiableRow, Clone)]
pub struct DeviceCategoriesDatabaseTableRow {
    pub id: i32,
    pub display_name: String,
}

impl GenerateStaticTableData for DeviceCategoriesDatabaseTable {
    const ITEMS: &[&str] = &[
        "Phone",
        "Tablet",
        "Desktop",
        "Laptop",
        "Game Console",
        "Camera",
        "Drone",
    ];
}

impl GenerateStaticRowData for DeviceCategoriesDatabaseTableRow {
    fn new(id: i32, display_name: impl Into<String>) -> Self {
        Self {
            id,
            display_name: display_name.into(),
        }
    }
}
