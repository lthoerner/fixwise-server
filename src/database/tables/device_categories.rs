use sqlx::query_builder::Separated;
use sqlx::Postgres;

use proc_macros::DatabaseEntity;

use super::IdentifiableRow;
use crate::database::{BulkInsert, DatabaseEntity, GenerateStaticRowData, GenerateStaticTableData};

#[derive(DatabaseEntity)]
#[entity(entity_name = "device_categories", primary_column = "id")]
pub struct DeviceCategoriesDatabaseTable {
    rows: Vec<DeviceCategoriesDatabaseTableRow>,
}

impl BulkInsert for DeviceCategoriesDatabaseTable {
    const COLUMN_NAMES: &[&str] = &["id", "display_name"];
    fn push_bindings(mut builder: Separated<Postgres, &str>, row: Self::Row) {
        builder.push_bind(row.id).push_bind(row.display_name);
    }
}

#[derive(sqlx::FromRow, Clone)]
pub struct DeviceCategoriesDatabaseTableRow {
    pub id: i32,
    pub display_name: String,
}

impl IdentifiableRow for DeviceCategoriesDatabaseTableRow {
    fn id(&self) -> i32 {
        self.id
    }
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
    fn new(id: i32, display_name: String) -> Self {
        Self { id, display_name }
    }
}
