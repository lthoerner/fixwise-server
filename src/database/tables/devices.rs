use std::collections::HashSet;

use super::customers::CustomersDatabaseTable;
use super::device_models::DeviceModelsDatabaseTable;
use super::generators::*;
use super::IdentifiableRow;
use crate::database::DatabaseEntity;

pub struct DevicesDatabaseTable {
    rows: Vec<DevicesDatabaseTableRow>,
}

impl DatabaseEntity for DevicesDatabaseTable {
    type Row = DevicesDatabaseTableRow;
    const ENTITY_NAME: &'static str = "devices";
    const PRIMARY_COLUMN_NAME: &'static str = "id";

    fn with_rows(rows: Vec<Self::Row>) -> Self {
        Self { rows }
    }

    fn rows(self) -> Vec<Self::Row> {
        self.rows
    }

    fn borrow_rows(&self) -> &[Self::Row] {
        &self.rows
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
