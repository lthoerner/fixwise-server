use proc_macros::{BulkInsert, DatabaseEntity, IdentifiableRow};

use crate::database::{GenerateStaticRowData, GenerateStaticTableData};

#[derive(DatabaseEntity, BulkInsert)]
#[entity(
    entity_name = "part_categories",
    primary_column = "id",
    columns = ["id", "display_name"]
)]
pub struct PartCategoriesDatabaseTable {
    rows: Vec<PartCategoriesDatabaseTableRow>,
}

#[derive(sqlx::FromRow, Clone, IdentifiableRow)]
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
