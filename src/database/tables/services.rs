use std::collections::HashSet;

use proc_macros::{BulkInsert, DatabaseEntity, GenerateTableData, IdentifiableRow, SingleInsert};

use super::device_models::DeviceModelsDatabaseTable;
use super::generators::*;
use super::service_types::ServiceTypesDatabaseTable;
use super::IdentifiableRow;
use crate::database::{DatabaseEntity, GenerateRowData};

#[derive(DatabaseEntity, BulkInsert, GenerateTableData, Clone)]
#[entity(entity_name = "services", primary_key = "id")]
pub struct ServicesDatabaseTable {
    rows: Vec<ServicesDatabaseTableRow>,
}

#[derive(SingleInsert, sqlx::FromRow, IdentifiableRow, Clone)]
pub struct ServicesDatabaseTableRow {
    pub id: i32,
    #[sqlx(rename = "type")]
    pub r#type: i32,
    pub device: i32,
}

impl GenerateRowData for ServicesDatabaseTableRow {
    type Identifier = i32;
    type Dependencies<'a> = (&'a ServiceTypesDatabaseTable, &'a DeviceModelsDatabaseTable);
    fn generate(
        _existing_rows: &[Self],
        existing_ids: &mut HashSet<Self::Identifier>,
        dependencies: Self::Dependencies<'_>,
    ) -> Self {
        Self {
            id: generate_unique_i32(0, existing_ids),
            r#type: dependencies.0.pick_random().id(),
            device: dependencies.1.pick_random().id(),
        }
    }
}
