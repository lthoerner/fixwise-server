use super::IdentifiableRow;
use crate::database::DatabaseEntity;

pub struct PartManufacturersDatabaseTable {
    rows: Vec<PartManufacturersDatabaseTableRow>,
}

impl DatabaseEntity for PartManufacturersDatabaseTable {
    type Row = PartManufacturersDatabaseTableRow;
    const ENTITY_NAME: &'static str = "part_manufacturers";
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
pub struct PartManufacturersDatabaseTableRow {
    pub id: i32,
    pub display_name: String,
}

impl IdentifiableRow for PartManufacturersDatabaseTableRow {
    fn id(&self) -> i32 {
        self.id
    }
}
