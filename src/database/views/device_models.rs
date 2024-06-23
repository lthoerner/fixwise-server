use crate::database::DatabaseEntity;

pub struct DeviceModelsDatabaseView {
    rows: Vec<DeviceModelsDatabaseViewRow>,
}

impl DatabaseEntity for DeviceModelsDatabaseView {
    type Row = DeviceModelsDatabaseViewRow;
    const ENTITY_NAME: &str = "device_models_view";
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

#[derive(sqlx::FromRow, Clone)]
pub struct DeviceModelsDatabaseViewRow {
    id: i32,
    display_name: String,
    manufacturer: String,
    category: String,
}
