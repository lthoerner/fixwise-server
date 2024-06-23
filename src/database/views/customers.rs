use crate::database::DatabaseEntity;

pub struct CustomersDatabaseView {
    rows: Vec<CustomersDatabaseViewRow>,
}

impl DatabaseEntity for CustomersDatabaseView {
    type Row = CustomersDatabaseViewRow;
    const ENTITY_NAME: &str = "customers_view";
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
pub struct CustomersDatabaseViewRow {
    pub id: i32,
    pub name: String,
    pub email_address: Option<String>,
    pub phone_number: Option<String>,
    pub street_address: Option<String>,
}
