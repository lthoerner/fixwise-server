use sqlx::query_builder::Separated;
use sqlx::Postgres;

use super::IdentifiableRow;
use crate::database::{BulkInsert, DatabaseEntity};

pub struct TypeAllocationCodesDatabaseTable {
    rows: Vec<TypeAllocationCodesDatabaseTableRow>,
}

impl DatabaseEntity for TypeAllocationCodesDatabaseTable {
    type Row = TypeAllocationCodesDatabaseTableRow;
    const SCHEMA_NAME: &str = "persistent";
    const ENTITY_NAME: &str = "type_allocation_codes";
    const PRIMARY_COLUMN_NAME: &str = "tac";

    // TODO: Take `Into<Vec<Self::Row>>` here
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

impl BulkInsert for TypeAllocationCodesDatabaseTable {
    const COLUMN_NAMES: &[&str] = &["tac", "manufacturer", "model"];
    fn push_bindings(mut builder: Separated<Postgres, &str>, row: Self::Row) {
        builder
            .push_bind(row.tac)
            .push_bind(row.manufacturer)
            .push_bind(row.model);
    }
}

#[derive(sqlx::FromRow, Clone)]
pub struct TypeAllocationCodesDatabaseTableRow {
    pub tac: i32,
    pub manufacturer: String,
    pub model: String,
}

impl IdentifiableRow for TypeAllocationCodesDatabaseTableRow {
    fn id(&self) -> i32 {
        self.tac
    }
}
