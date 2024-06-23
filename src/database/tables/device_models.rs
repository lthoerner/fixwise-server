use sqlx::query_builder::Separated;
use sqlx::Postgres;

use super::IdentifiableRow;
use crate::database::DatabaseEntity;

pub struct DeviceModelsDatabaseTable {
    rows: Vec<DeviceModelsDatabaseTableRow>,
}

impl DatabaseEntity for DeviceModelsDatabaseTable {
    type Row = DeviceModelsDatabaseTableRow;
    const ENTITY_NAME: &str = "device_models";
    const COLUMN_NAMES: &[&str] = &[
        "id",
        "display_name",
        "primary_model_identifiers",
        "secondary_model_identifiers",
        "manufacturer",
        "category",
    ];
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
