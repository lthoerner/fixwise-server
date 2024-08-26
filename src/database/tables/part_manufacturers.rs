use std::collections::HashSet;

use proc_macros::{BulkInsert, DatabaseEntity, IdentifiableRow};

use super::generators::*;
use crate::database::{GenerateRowData, GenerateTableData};

#[derive(DatabaseEntity, BulkInsert)]
#[entity(
    entity_name = "part_manufacturers",
    primary_column = "id",
    columns = ["id", "display_name"]
)]
pub struct PartManufacturersDatabaseTable {
    rows: Vec<PartManufacturersDatabaseTableRow>,
}

#[derive(sqlx::FromRow, Clone, IdentifiableRow)]
pub struct PartManufacturersDatabaseTableRow {
    pub id: i32,
    pub display_name: String,
}

impl GenerateTableData for PartManufacturersDatabaseTable {}
impl GenerateRowData for PartManufacturersDatabaseTableRow {
    type Identifier = i32;
    type Dependencies<'a> = ();
    fn generate(
        existing_ids: &mut HashSet<Self::Identifier>,
        _dependencies: Self::Dependencies<'_>,
    ) -> Self {
        Self {
            id: generate_unique_i32(0, existing_ids),
            display_name: generate_company_name(),
        }
    }
}
