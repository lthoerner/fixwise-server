use std::collections::HashSet;

use sqlx::query_builder::Separated;
use sqlx::Postgres;

use super::customers::CustomersDatabaseTable;
use super::device_models::DeviceModelsDatabaseTable;
use super::generators::*;
use super::IdentifiableRow;
use crate::database::{BulkInsert, DatabaseEntity};

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

impl DevicesDatabaseTable {
    fn generate(
        count: usize,
        existing_device_models: &DeviceModelsDatabaseTable,
        existing_customers: &CustomersDatabaseTable,
    ) -> Self {
        let mut rows = Vec::new();
        let mut existing_ids = HashSet::new();
        for _ in 0..count {
            rows.push(DevicesDatabaseTableRow::generate(
                &mut existing_ids,
                existing_device_models,
                existing_customers,
            ));
        }

        Self::with_rows(rows)
    }
}

impl DevicesDatabaseTableRow {
    fn generate(
        existing_ids: &mut HashSet<i32>,
        existing_device_models: &DeviceModelsDatabaseTable,
        existing_customers: &CustomersDatabaseTable,
    ) -> Self {
        Self {
            id: generate_unique_i32(0, existing_ids),
            model: existing_device_models.pick_random().id(),
            owner: generate_option(existing_customers.pick_random().id(), 0.9),
        }
    }
}
