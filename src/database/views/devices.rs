use crate::database::DatabaseEntity;

pub struct DevicesDatabaseView {
    rows: Vec<DevicesDatabaseViewRow>,
}

impl DatabaseEntity for DevicesDatabaseView {
    type Row = DevicesDatabaseViewRow;
    const ENTITY_NAME: &str = "devices_view";
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
pub struct DevicesDatabaseViewRow {
    pub id: i32,
    pub model: String,
    pub owner: Option<String>,
}
