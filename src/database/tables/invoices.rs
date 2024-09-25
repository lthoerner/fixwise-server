use std::collections::HashSet;

use chrono::NaiveDateTime;

use proc_macros::{BulkInsert, DatabaseEntity, GenerateTableData, IdentifiableRow, SingleInsert};

use super::generators::*;
use crate::database::GenerateRowData;

#[derive(DatabaseEntity, BulkInsert, GenerateTableData, Clone)]
#[entity(
    entity_name = "invoices",
    primary_key = "id",
    foreign_key_name = "invoice"
)]
pub struct InvoicesDatabaseTable {
    rows: Vec<InvoicesDatabaseTableRow>,
}

#[derive(SingleInsert, sqlx::FromRow, IdentifiableRow, Clone)]
pub struct InvoicesDatabaseTableRow {
    pub id: i32,
    #[defaultable]
    pub created_at: Option<NaiveDateTime>,
    #[defaultable]
    pub updated_at: Option<NaiveDateTime>,
}

impl GenerateRowData for InvoicesDatabaseTableRow {
    type Identifier = i32;
    type Dependencies<'a> = ();
    fn generate(
        _existing_rows: &[Self],
        existing_ids: &mut HashSet<Self::Identifier>,
        _dependencies: Self::Dependencies<'_>,
    ) -> Self {
        let created_at = generate_date(None);
        let updated_at = generate_date(Some(created_at));

        Self {
            id: generate_unique_i32(0, existing_ids),
            created_at: Some(created_at),
            updated_at: Some(updated_at),
        }
    }
}
