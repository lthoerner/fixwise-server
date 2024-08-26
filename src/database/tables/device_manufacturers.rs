use std::collections::HashSet;

use sqlx::query_builder::Separated;
use sqlx::Postgres;

use proc_macros::{DatabaseEntity, IdentifiableRow};

use super::generators::*;
use super::IdentifiableRow;
use crate::database::{BulkInsert, DatabaseEntity, GenerateRowData, GenerateTableData};

#[derive(DatabaseEntity)]
#[entity(entity_name = "device_manufacturers", primary_column = "id")]
pub struct DeviceManufacturersDatabaseTable {
    rows: Vec<DeviceManufacturersDatabaseTableRow>,
}

impl BulkInsert for DeviceManufacturersDatabaseTable {
    const COLUMN_NAMES: &[&str] = &["id", "display_name"];
    fn push_bindings(mut builder: Separated<Postgres, &str>, row: Self::Row) {
        builder.push_bind(row.id).push_bind(row.display_name);
    }
}

#[derive(sqlx::FromRow, Clone, IdentifiableRow)]
pub struct DeviceManufacturersDatabaseTableRow {
    pub id: i32,
    pub display_name: String,
}

impl GenerateTableData for DeviceManufacturersDatabaseTable {}
impl GenerateRowData for DeviceManufacturersDatabaseTableRow {
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
