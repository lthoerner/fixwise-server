use super::IdentifiableRow;
use crate::database::DatabaseEntity;

pub struct VendorsDatabaseTable {
    rows: Vec<VendorsDatabaseTableRow>,
}

impl DatabaseEntity for VendorsDatabaseTable {
    type Row = VendorsDatabaseTableRow;
    const ENTITY_NAME: &'static str = "vendors";
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
pub struct VendorsDatabaseTableRow {
    pub id: i32,
    pub display_name: String,
    pub email_address: Option<String>,
    pub phone_number: Option<String>,
    pub street_address: Option<String>,
}

impl IdentifiableRow for VendorsDatabaseTableRow {
    fn id(&self) -> i32 {
        self.id
    }
}
