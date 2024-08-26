use sqlx::query_builder::Separated;
use sqlx::Postgres;

use proc_macros::DatabaseEntity;

use super::IdentifiableRow;
use crate::database::{BulkInsert, DatabaseEntity};

#[derive(DatabaseEntity)]
#[entity(
    schema_name = "persistent",
    entity_name = "type_allocation_codes",
    primary_column = "tac"
)]
pub struct TypeAllocationCodesDatabaseTable {
    rows: Vec<TypeAllocationCodesDatabaseTableRow>,
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

#[derive(sqlx::FromRow, Clone, Debug)]
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
