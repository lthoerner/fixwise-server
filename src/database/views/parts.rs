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
    id: i32,
    display_name: String,
    vendor: String,
    manufacturer: Option<String>,
    category: String,
    cost: Option<Decimal>,
    price: Option<Decimal>,
}
