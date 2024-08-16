use super::IdentifiableRow;
use crate::database::DatabaseEntity;

pub struct TypeAllocationCodesDatabaseTable {
    rows: Vec<TypeAllocationCodesDatabaseTableRow>,
}

impl DatabaseEntity for TypeAllocationCodesDatabaseTable {
    type Row = TypeAllocationCodesDatabaseTableRow;
    const ENTITY_NAME: &str = "type_allocation_codes";
    const PRIMARY_COLUMN_NAME: &str = "tac";

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
