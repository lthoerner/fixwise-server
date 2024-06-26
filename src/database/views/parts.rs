use rust_decimal::Decimal;

use crate::database::DatabaseEntity;

pub struct PartsDatabaseView {
    rows: Vec<PartsDatabaseViewRow>,
}

impl DatabaseEntity for PartsDatabaseView {
    type Row = PartsDatabaseViewRow;
    const ENTITY_NAME: &str = "parts_view";
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
pub struct PartsDatabaseViewRow {
    pub id: i32,
    pub display_name: String,
    pub vendor: String,
    pub manufacturer: Option<String>,
    pub category: String,
    pub cost: Option<Decimal>,
    pub price: Option<Decimal>,
}
