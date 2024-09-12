use std::collections::HashSet;

use proc_macros::{BulkInsert, DatabaseEntity, GenerateTableData, IdentifiableRow, SingleInsert};

use super::generators::*;
use crate::database::GenerateRowData;

#[derive(DatabaseEntity, BulkInsert, GenerateTableData)]
#[entity(entity_name = "device_manufacturers", primary_key = "id")]
pub struct DeviceManufacturersDatabaseTable {
    rows: Vec<DeviceManufacturersDatabaseTableRow>,
}

#[derive(SingleInsert, sqlx::FromRow, Clone, IdentifiableRow)]
pub struct DeviceManufacturersDatabaseTableRow {
    pub id: i32,
    pub display_name: String,
}

impl GenerateRowData for DeviceManufacturersDatabaseTableRow {
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
        }
    }
}
