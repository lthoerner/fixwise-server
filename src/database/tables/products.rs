use std::collections::HashSet;

use proc_macros::{BulkInsert, DatabaseEntity, GenerateTableData, IdentifiableRow, SingleInsert};

use super::generators::*;
use crate::database::GenerateRowData;

#[derive(DatabaseEntity, BulkInsert, GenerateTableData, Clone)]
#[entity(entity_name = "products", primary_key = "sku")]
pub struct ProductsDatabaseTable {
    rows: Vec<ProductsDatabaseTableRow>,
}

#[derive(SingleInsert, sqlx::FromRow, IdentifiableRow, Clone)]
pub struct ProductsDatabaseTableRow {
    pub sku: i32,
    pub display_name: String,
}

impl GenerateRowData for ProductsDatabaseTableRow {
    type Identifier = i32;
    type Dependencies<'a> = ();
    fn generate(
        _existing_rows: &[Self],
        existing_ids: &mut HashSet<Self::Identifier>,
        _dependencies: Self::Dependencies<'_>,
    ) -> Self {
        Self {
            sku: generate_unique_i32(0, existing_ids),
            display_name: "PLACEHOLDER".to_owned(),
        }
    }
}
