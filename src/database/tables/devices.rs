use std::collections::HashSet;

use proc_macros::{BulkInsert, DatabaseEntity, GenerateTableData, IdentifiableRow, SingleInsert};

use super::customers::CustomersDatabaseTable;
use super::device_models::DeviceModelsDatabaseTable;
use super::generators::*;
use super::IdentifiableRow;
use crate::database::{DatabaseEntity, GenerateRowData};

#[derive(DatabaseEntity, BulkInsert, GenerateTableData, Clone)]
#[entity(
    entity_name = "devices",
    primary_key = "id",
    foreign_key_name = "device"
)]
pub struct DevicesDatabaseTable {
    rows: Vec<DevicesDatabaseTableRow>,
}

#[derive(SingleInsert, sqlx::FromRow, IdentifiableRow, Clone)]
pub struct DevicesDatabaseTableRow {
    pub id: i32,
    pub model: i32,
    pub owner: Option<i32>,
}

impl GenerateRowData for DevicesDatabaseTableRow {
    type Identifier = i32;
    type Dependencies<'a> = (&'a DeviceModelsDatabaseTable, &'a CustomersDatabaseTable);
    fn generate(
        _existing_rows: &[Self],
        existing_ids: &mut HashSet<Self::Identifier>,
        dependencies: Self::Dependencies<'_>,
    ) -> Self {
        Self {
            id: generate_unique_i32(0, existing_ids),
            model: dependencies.0.pick_random().id(),
            owner: generate_option(dependencies.1.pick_random().id(), 0.9),
        }
    }
}
