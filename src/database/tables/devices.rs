use std::collections::HashSet;

use sqlx::query_builder::Separated;
use sqlx::Postgres;

use proc_macros::{DatabaseEntity, IdentifiableRow};

use super::customers::CustomersDatabaseTable;
use super::device_models::DeviceModelsDatabaseTable;
use super::generators::*;
use super::IdentifiableRow;
use crate::database::{BulkInsert, DatabaseEntity, GenerateRowData, GenerateTableData};

#[derive(DatabaseEntity)]
#[entity(entity_name = "devices", primary_column = "id")]
pub struct DevicesDatabaseTable {
    rows: Vec<DevicesDatabaseTableRow>,
}

impl BulkInsert for DevicesDatabaseTable {
    const COLUMN_NAMES: &[&str] = &["id", "model", "owner"];
    fn push_bindings(mut builder: Separated<Postgres, &str>, row: Self::Row) {
        builder
            .push_bind(row.id)
            .push_bind(row.model)
            .push_bind(row.owner);
    }
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
