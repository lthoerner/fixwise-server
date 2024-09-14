use std::collections::HashSet;

use proc_macros::{BulkInsert, DatabaseEntity, GenerateTableData, IdentifiableRow, SingleInsert};

use super::device_categories::DeviceCategoriesDatabaseTable;
use super::device_manufacturers::DeviceManufacturersDatabaseTable;
use super::generators::*;
use super::IdentifiableRow;
use crate::database::{DatabaseEntity, GenerateRowData};

#[derive(DatabaseEntity, BulkInsert, GenerateTableData, Clone)]
#[entity(entity_name = "device_models", primary_key = "id")]
pub struct DeviceModelsDatabaseTable {
    rows: Vec<DeviceModelsDatabaseTableRow>,
}

#[derive(SingleInsert, sqlx::FromRow, IdentifiableRow, Clone)]
pub struct DeviceModelsDatabaseTableRow {
    pub id: i32,
    pub display_name: String,
    pub primary_model_identifiers: Vec<String>,
    pub secondary_model_identifiers: Vec<String>,
    pub manufacturer: i32,
    pub category: i32,
}

impl GenerateRowData for DeviceModelsDatabaseTableRow {
    type Identifier = i32;
    type Dependencies<'a> = (
        &'a DeviceManufacturersDatabaseTable,
        &'a DeviceCategoriesDatabaseTable,
    );
    fn generate(
        _existing_rows: &[Self],
        existing_ids: &mut HashSet<Self::Identifier>,
        dependencies: Self::Dependencies<'_>,
    ) -> Self {
        Self {
            id: generate_unique_i32(0, existing_ids),
            display_name: generate_device_name(),
            // TODO: Add model identifiers generator
            primary_model_identifiers: Vec::new(),
            secondary_model_identifiers: Vec::new(),
            manufacturer: dependencies.0.pick_random().id(),
            category: dependencies.1.pick_random().id(),
        }
    }
}
