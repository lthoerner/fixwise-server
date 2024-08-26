use std::collections::HashSet;

use sqlx::query_builder::Separated;
use sqlx::Postgres;

use proc_macros::{DatabaseEntity, IdentifiableRow};

use super::generators::*;
use super::IdentifiableRow;
use crate::database::{BulkInsert, DatabaseEntity, GenerateRowData, GenerateTableData};

#[derive(DatabaseEntity)]
#[entity(entity_name = "vendors", primary_column = "id")]
pub struct VendorsDatabaseTable {
    rows: Vec<VendorsDatabaseTableRow>,
}

impl BulkInsert for VendorsDatabaseTable {
    const COLUMN_NAMES: &[&str] = &[
        "id",
        "display_name",
        "email_address",
        "phone_number",
        "street_address",
    ];

    fn push_bindings(mut builder: Separated<Postgres, &str>, row: Self::Row) {
        builder
            .push_bind(row.id)
            .push_bind(row.display_name)
            .push_bind(row.email_address)
            .push_bind(row.phone_number)
            .push_bind(row.street_address);
    }
}

#[derive(sqlx::FromRow, Clone, IdentifiableRow)]
pub struct VendorsDatabaseTableRow {
    pub id: i32,
    pub display_name: String,
    pub email_address: Option<String>,
    pub phone_number: Option<String>,
    pub street_address: Option<String>,
}

impl GenerateTableData for VendorsDatabaseTable {}
impl GenerateRowData for VendorsDatabaseTableRow {
    type Identifier = i32;
    type Dependencies<'a> = ();
    fn generate(
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
