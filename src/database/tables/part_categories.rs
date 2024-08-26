use sqlx::query_builder::Separated;
use sqlx::Postgres;

use proc_macros::{DatabaseEntity, IdentifiableRow};

use crate::database::{BulkInsert, GenerateStaticRowData, GenerateStaticTableData};

#[derive(DatabaseEntity)]
#[entity(entity_name = "part_categories", primary_column = "id")]
pub struct PartCategoriesDatabaseTable {
    rows: Vec<PartCategoriesDatabaseTableRow>,
}

impl BulkInsert for PartCategoriesDatabaseTable {
    const COLUMN_NAMES: &[&str] = &["id", "display_name"];
    fn push_bindings(mut builder: Separated<Postgres, &str>, row: Self::Row) {
        builder.push_bind(row.id).push_bind(row.display_name);
    }
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
