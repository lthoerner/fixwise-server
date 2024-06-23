use crate::database::DatabaseEntity;

pub struct VendorsDatabaseView {
    rows: Vec<VendorsDatabaseViewRow>,
}

impl DatabaseEntity for VendorsDatabaseView {
    type Row = VendorsDatabaseViewRow;
    const ENTITY_NAME: &str = "vendors_view";
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
pub struct VendorsDatabaseViewRow {
    id: i32,
    display_name: String,
}
