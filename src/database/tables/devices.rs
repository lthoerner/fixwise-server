use std::collections::HashSet;

use sqlx::query_builder::Separated;
use sqlx::Postgres;

use super::customers::CustomersDatabaseTable;
use super::device_models::DeviceModelsDatabaseTable;
use super::generators::*;
use super::IdentifiableRow;
use crate::database::{BulkInsert, DatabaseEntity, GenerateRowData, GenerateTableData};

pub struct DevicesDatabaseTable {
    rows: Vec<DevicesDatabaseTableRow>,
}

impl DatabaseEntity for DevicesDatabaseTable {
    type Row = DevicesDatabaseTableRow;
    const ENTITY_NAME: &str = "devices";
    const PRIMARY_COLUMN_NAME: &str = "id";

    fn with_rows(rows: Vec<Self::Row>) -> Self {
        Self { rows }
    }

    fn take_rows(self) -> Vec<Self::Row> {
        self.rows
    }

    fn rows(&self) -> &[Self::Row] {
        &self.rows
    }
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

#[derive(sqlx::FromRow, Clone)]
pub struct DevicesDatabaseTableRow {
    pub id: i32,
    pub model: i32,
    pub owner: Option<i32>,
}

impl IdentifiableRow for DevicesDatabaseTableRow {
    fn id(&self) -> i32 {
        self.id
    }
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
