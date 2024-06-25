use std::collections::HashSet;

use sqlx::query_builder::Separated;
use sqlx::Postgres;

use super::generators::*;
use super::IdentifiableRow;
use crate::database::{BulkInsert, DatabaseEntity};

pub struct DeviceCategoriesDatabaseTable {
    rows: Vec<DeviceCategoriesDatabaseTableRow>,
}

impl DatabaseEntity for DeviceCategoriesDatabaseTable {
    type Row = DeviceCategoriesDatabaseTableRow;
    const ENTITY_NAME: &str = "device_categories";
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

impl BulkInsert for DeviceCategoriesDatabaseTable {
    const COLUMN_NAMES: &[&str] = &["id", "display_name"];

    fn push_bindings(mut builder: Separated<Postgres, &str>, row: Self::Row) {
        builder.push_bind(row.id).push_bind(row.display_name);
    }
}

#[derive(sqlx::FromRow, Clone)]
pub struct DeviceCategoriesDatabaseTableRow {
    pub id: i32,
    pub display_name: String,
}

impl IdentifiableRow for DeviceCategoriesDatabaseTableRow {
    fn id(&self) -> i32 {
        self.id
    }
}

impl DeviceCategoriesDatabaseTable {
    pub fn generate() -> Self {
        const DEVICE_CATEGORIES: [&str; 7] = [
            "Phone",
            "Tablet",
            "Desktop",
            "Laptop",
            "Game Console",
            "Camera",
            "Drone",
        ];

        let mut existing_ids = HashSet::new();

        let rows = DEVICE_CATEGORIES
            .iter()
            .map(|category| DeviceCategoriesDatabaseTableRow {
                id: generate_unique_i32(0, &mut existing_ids),
                display_name: (*category).to_owned(),
            })
            .collect();

        Self::with_rows(rows)
    }
}
