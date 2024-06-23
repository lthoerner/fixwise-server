use sqlx::query_builder::Separated;
use sqlx::Postgres;

use super::IdentifiableRow;
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
