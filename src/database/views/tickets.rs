use chrono::NaiveDateTime;
use rust_decimal::Decimal;
use sqlx::FromRow;

use crate::database::DatabaseEntity;

pub struct TicketsDatabaseView {
    rows: Vec<TicketsDatabaseViewRow>,
}

impl DatabaseEntity for TicketsDatabaseView {
    type Row = TicketsDatabaseViewRow;
    const ENTITY_NAME: &'static str = "tickets_view";
    const PRIMARY_COLUMN_NAME: &'static str = "id";

    fn with_rows(rows: Vec<Self::Row>) -> Self {
        Self { rows }
    }

    fn rows(self) -> Vec<Self::Row> {
        self.rows
    }
}

#[derive(FromRow)]
pub struct TicketsDatabaseViewRow {
    pub id: i32,
    pub customer_name: String,
    pub device: String,
    pub balance: Decimal,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}
