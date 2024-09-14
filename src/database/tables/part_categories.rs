use proc_macros::{BulkInsert, DatabaseEntity, IdentifiableRow, SingleInsert};

use crate::database::{GenerateStaticRowData, GenerateStaticTableData};

#[derive(DatabaseEntity, BulkInsert, Clone)]
#[entity(entity_name = "part_categories", primary_key = "id")]
pub struct PartCategoriesDatabaseTable {
    rows: Vec<PartCategoriesDatabaseTableRow>,
}

#[derive(SingleInsert, sqlx::FromRow, IdentifiableRow, Clone)]
pub struct PartCategoriesDatabaseTableRow {
    pub id: i32,
    pub display_name: String,
}

impl GenerateStaticTableData for PartCategoriesDatabaseTable {
    const ITEMS: &[&str] = &[
        "Screen",
        "Battery",
        "Backglass",
        "Frame",
        "Front Camera",
        "Rear Camera",
        "Charge Port",
    ];
}

impl GenerateStaticRowData for PartCategoriesDatabaseTableRow {
    fn new(id: i32, display_name: impl Into<String>) -> Self {
        Self {
            id,
            display_name: display_name.into(),
        }
    }
}
