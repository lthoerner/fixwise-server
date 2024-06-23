use sqlx::query_builder::Separated;
use sqlx::Postgres;

use super::IdentifiableRow;
use crate::database::DatabaseEntity;

pub struct PartCategoriesDatabaseTable {
    rows: Vec<PartCategoriesDatabaseTableRow>,
}

impl DatabaseEntity for PartCategoriesDatabaseTable {
    type Row = PartCategoriesDatabaseTableRow;
    const ENTITY_NAME: &str = "part_categories";
    const COLUMN_NAMES: &[&str] = &["id", "display_name"];
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
