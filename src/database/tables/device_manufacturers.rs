use std::collections::HashSet;

use sqlx::query_builder::Separated;
use sqlx::Postgres;

use super::generators::*;
use super::IdentifiableRow;
use crate::database::loading_bar::LoadingBar;
use crate::database::{BulkInsert, DatabaseEntity};

pub struct DeviceManufacturersDatabaseTable {
    rows: Vec<DeviceManufacturersDatabaseTableRow>,
}

impl DatabaseEntity for DeviceManufacturersDatabaseTable {
    type Row = DeviceManufacturersDatabaseTableRow;
    const ENTITY_NAME: &str = "device_manufacturers";
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

impl BulkInsert for DeviceManufacturersDatabaseTable {
    const COLUMN_NAMES: &[&str] = &["id", "display_name"];

    fn push_bindings(mut builder: Separated<Postgres, &str>, row: Self::Row) {
        builder.push_bind(row.id).push_bind(row.display_name);
    }
}

#[derive(sqlx::FromRow, Clone)]
pub struct DeviceManufacturersDatabaseTableRow {
    pub id: i32,
    pub display_name: String,
}

impl IdentifiableRow for DeviceManufacturersDatabaseTableRow {
    fn id(&self) -> i32 {
        self.id
    }
}

impl DeviceManufacturersDatabaseTable {
    pub fn generate(count: usize) -> Self {
        let mut rows = Vec::new();
        let mut existing_ids = HashSet::new();
        let mut loading_bar = LoadingBar::new(count);
        for _ in 0..count {
            loading_bar.update();
            rows.push(DeviceManufacturersDatabaseTableRow::generate(
                &mut existing_ids,
            ));
        }

        Self::with_rows(rows)
    }
}

impl DeviceManufacturersDatabaseTableRow {
    fn generate(existing_ids: &mut HashSet<i32>) -> Self {
        Self {
            id: generate_unique_i32(0, existing_ids),
            display_name: generate_company_name(),
        }
    }
}
