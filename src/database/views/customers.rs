use sqlx::FromRow;

use crate::database::DatabaseEntity;

pub struct CustomersDatabaseView {
    rows: Vec<CustomersDatabaseViewRow>,
}

impl DatabaseEntity for CustomersDatabaseView {
    type Row = CustomersDatabaseViewRow;
    const ENTITY_NAME: &'static str = "customers_view";
    const PRIMARY_COLUMN_NAME: &'static str = "id";

    fn with_rows(rows: Vec<Self::Row>) -> Self {
        Self { rows }
    }

    fn rows(self) -> Vec<Self::Row> {
        self.rows
    }
}

#[derive(FromRow)]
pub struct CustomersDatabaseViewRow {
    pub id: i32,
    pub name: String,
    pub email: String,
    pub phone: String,
    pub address: Option<String>,
}
