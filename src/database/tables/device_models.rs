use super::IdentifiableRow;
use crate::database::DatabaseEntity;

pub struct DeviceModelsDatabaseTable {
    rows: Vec<DeviceModelsDatabaseTableRow>,
}

impl DatabaseEntity for DeviceModelsDatabaseTable {
    type Row = DeviceModelsDatabaseTableRow;
    const ENTITY_NAME: &'static str = "device_models";
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
