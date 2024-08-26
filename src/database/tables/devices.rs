use std::collections::HashSet;

use proc_macros::{BulkInsert, DatabaseEntity, IdentifiableRow};

use super::customers::CustomersDatabaseTable;
use super::device_models::DeviceModelsDatabaseTable;
use super::generators::*;
use super::IdentifiableRow;
use crate::database::{DatabaseEntity, GenerateRowData, GenerateTableData};

#[derive(DatabaseEntity, BulkInsert)]
#[entity(
    entity_name = "devices",
    primary_column = "id",
    columns = ["id", "model", "owner"]
)]
pub struct DevicesDatabaseTable {
    rows: Vec<DevicesDatabaseTableRow>,
}

#[derive(sqlx::FromRow, Clone, IdentifiableRow)]
pub struct DevicesDatabaseTableRow {
    pub id: i32,
    pub model: i32,
    pub owner: Option<i32>,
}

impl GenerateTableData for DevicesDatabaseTable {}
impl GenerateRowData for DevicesDatabaseTableRow {
    type Identifier = i32;
    type Dependencies<'a> = (&'a DeviceModelsDatabaseTable, &'a CustomersDatabaseTable);
    fn generate(
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
