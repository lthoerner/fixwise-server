use super::IdentifiableRow;
use crate::database::DatabaseEntity;

pub struct DeviceCategoriesDatabaseTable {
    rows: Vec<DeviceCategoriesDatabaseTableRow>,
}

impl DatabaseEntity for DeviceCategoriesDatabaseTable {
    type Row = DeviceCategoriesDatabaseTableRow;
    const ENTITY_NAME: &'static str = "device_categories";
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
pub struct DeviceCategoriesDatabaseTableRow {
    pub id: i32,
    pub display_name: String,
}

impl IdentifiableRow for DeviceCategoriesDatabaseTableRow {
    fn id(&self) -> i32 {
        self.id
    }
}
