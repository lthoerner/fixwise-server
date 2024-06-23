use super::IdentifiableRow;
use crate::database::DatabaseEntity;

pub struct DeviceManufacturersDatabaseTable {
    rows: Vec<DeviceManufacturersDatabaseTableRow>,
}

impl DatabaseEntity for DeviceManufacturersDatabaseTable {
    type Row = DeviceManufacturersDatabaseTableRow;
    const ENTITY_NAME: &'static str = "device_manufacturers";
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
pub struct DeviceManufacturersDatabaseTableRow {
    pub id: i32,
    pub display_name: String,
}

impl IdentifiableRow for DeviceManufacturersDatabaseTableRow {
    fn id(&self) -> i32 {
        self.id
    }
}
