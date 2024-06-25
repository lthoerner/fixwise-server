use std::collections::HashSet;

use sqlx::query_builder::Separated;
use sqlx::Postgres;

use super::generators::*;
use super::IdentifiableRow;
use crate::database::{BulkInsert, DatabaseEntity};

pub struct PartCategoriesDatabaseTable {
    rows: Vec<PartCategoriesDatabaseTableRow>,
}

impl DatabaseEntity for PartCategoriesDatabaseTable {
    type Row = PartCategoriesDatabaseTableRow;
    const ENTITY_NAME: &str = "part_categories";
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

impl BulkInsert for PartCategoriesDatabaseTable {
    const COLUMN_NAMES: &[&str] = &["id", "display_name"];

    fn push_bindings(mut builder: Separated<Postgres, &str>, row: Self::Row) {
        builder.push_bind(row.id).push_bind(row.display_name);
    }
}

#[derive(sqlx::FromRow, Clone)]
pub struct PartCategoriesDatabaseTableRow {
    pub id: i32,
    pub display_name: String,
}

impl IdentifiableRow for PartCategoriesDatabaseTableRow {
    fn id(&self) -> i32 {
        self.id
    }
}

impl PartCategoriesDatabaseTable {
    pub fn generate() -> Self {
        const PART_CATEGORIES: [&str; 7] = [
            "Screen",
            "Battery",
            "Backglass",
            "Frame",
            "Front Camera",
            "Rear Camera",
            "Charge Port",
        ];

        let mut existing_ids = HashSet::new();

        let rows = PART_CATEGORIES
            .iter()
            .map(|category| PartCategoriesDatabaseTableRow {
                id: generate_unique_i32(0, &mut existing_ids),
                display_name: (*category).to_owned(),
            })
            .collect();

        Self::with_rows(rows)
    }
}