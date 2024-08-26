use std::collections::HashSet;

use sqlx::query_builder::Separated;
use sqlx::Postgres;

use proc_macros::{DatabaseEntity, IdentifiableRow};

use super::generators::*;
use crate::database::{BulkInsert, GenerateRowData, GenerateTableData};

#[derive(DatabaseEntity)]
#[entity(entity_name = "customers", primary_column = "id")]
pub struct CustomersDatabaseTable {
    rows: Vec<CustomersDatabaseTableRow>,
}

impl BulkInsert for CustomersDatabaseTable {
    const COLUMN_NAMES: &[&str] = &[
        "id",
        "name",
        "email_address",
        "phone_number",
        "street_address",
    ];

    fn push_bindings(mut builder: Separated<Postgres, &str>, row: Self::Row) {
        builder
            .push_bind(row.id)
            .push_bind(row.name)
            .push_bind(row.email_address)
            .push_bind(row.phone_number)
            .push_bind(row.street_address);
    }
}

#[derive(sqlx::FromRow, Clone, IdentifiableRow)]
pub struct CustomersDatabaseTableRow {
    pub id: i32,
    pub name: String,
    pub email_address: Option<String>,
    pub phone_number: Option<String>,
    pub street_address: Option<String>,
}

impl GenerateTableData for CustomersDatabaseTable {}
impl GenerateRowData for CustomersDatabaseTableRow {
    type Identifier = i32;
    type Dependencies<'a> = ();
    fn generate(
        existing_ids: &mut HashSet<Self::Identifier>,
        _dependencies: Self::Dependencies<'_>,
    ) -> Self {
        Self {
            id: generate_unique_i32(0, existing_ids),
            name: generate_name(),
            email_address: generate_option(generate_email_address(), 0.9),
            phone_number: generate_option(generate_phone_number(), 0.9),
            street_address: generate_option(generate_street_address(), 0.9),
        }
    }
}
