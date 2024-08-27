use proc_macros::{BulkInsert, DatabaseEntity, IdentifiableRow, SingleInsert};

use crate::database::{GenerateStaticRowData, GenerateStaticTableData};

#[derive(DatabaseEntity, BulkInsert)]
#[entity(entity_name = "part_categories", primary_column = "id")]
pub struct PartCategoriesDatabaseTable {
    rows: Vec<PartCategoriesDatabaseTableRow>,
}

#[derive(SingleInsert, sqlx::FromRow, Clone, IdentifiableRow)]
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
    fn new(id: i32, display_name: String) -> Self {
        Self { id, display_name }
    }
}
