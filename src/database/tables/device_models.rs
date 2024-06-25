use std::collections::HashSet;

use sqlx::query_builder::Separated;
use sqlx::Postgres;

use super::device_categories::DeviceCategoriesDatabaseTable;
use super::device_manufacturers::DeviceManufacturersDatabaseTable;
use super::generators::*;
use super::IdentifiableRow;
use crate::database::{BulkInsert, DatabaseEntity, GenerateRowData, GenerateTableData};

pub struct DeviceModelsDatabaseTable {
    rows: Vec<DeviceModelsDatabaseTableRow>,
}

impl DatabaseEntity for DeviceModelsDatabaseTable {
    type Row = DeviceModelsDatabaseTableRow;
    const ENTITY_NAME: &str = "device_models";
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

impl BulkInsert for DeviceModelsDatabaseTable {
    const COLUMN_NAMES: &[&str] = &[
        "id",
        "display_name",
        "primary_model_identifiers",
        "secondary_model_identifiers",
        "manufacturer",
        "category",
    ];

    fn push_bindings(mut builder: Separated<Postgres, &str>, row: Self::Row) {
        builder
            .push_bind(row.id)
            .push_bind(row.display_name)
            .push_bind(row.primary_model_identifiers)
            .push_bind(row.secondary_model_identifiers)
            .push_bind(row.manufacturer)
            .push_bind(row.category);
    }
}

#[derive(sqlx::FromRow, Clone)]
pub struct DeviceModelsDatabaseTableRow {
    pub id: i32,
    pub display_name: String,
    pub primary_model_identifiers: Vec<String>,
    pub secondary_model_identifiers: Vec<String>,
    pub manufacturer: i32,
    pub category: i32,
}

impl IdentifiableRow for DeviceModelsDatabaseTableRow {
    fn id(&self) -> i32 {
        self.id
    }
}

impl GenerateTableData for DeviceModelsDatabaseTable {}
impl GenerateRowData for DeviceModelsDatabaseTableRow {
    type Identifier = i32;
    type Dependencies<'a> = (
        &'a DeviceManufacturersDatabaseTable,
        &'a DeviceCategoriesDatabaseTable,
    );
    fn generate(
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
