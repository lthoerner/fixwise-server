use std::collections::HashSet;

use proc_macros::{BulkInsert, DatabaseEntity, GenerateTableData, IdentifiableRow, SingleInsert};

use super::generators::*;
use crate::database::GenerateRowData;

#[derive(DatabaseEntity, BulkInsert, GenerateTableData, Clone)]
#[entity(
    entity_name = "vendors",
    primary_key = "id",
    foreign_key_name = "vendor"
)]
pub struct VendorsDatabaseTable {
    rows: Vec<VendorsDatabaseTableRow>,
}

#[derive(SingleInsert, sqlx::FromRow, IdentifiableRow, Clone)]
pub struct VendorsDatabaseTableRow {
    pub id: i32,
    pub display_name: String,
    pub email_address: Option<String>,
    pub phone_number: Option<String>,
    pub street_address: Option<String>,
}

impl GenerateRowData for VendorsDatabaseTableRow {
    type Identifier = i32;
    type Dependencies<'a> = ();
    fn generate(
        _existing_rows: &[Self],
        existing_ids: &mut HashSet<Self::Identifier>,
        _dependencies: Self::Dependencies<'_>,
    ) -> Self {
        Self {
            id: generate_unique_i32(0, existing_ids),
            display_name: generate_company_name(),
            email_address: generate_option(generate_email_address(), 0.7),
            phone_number: generate_option(generate_phone_number(), 0.5),
            street_address: generate_option(generate_street_address(), 0.2),
        }
    }
}
