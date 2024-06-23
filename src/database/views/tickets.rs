use chrono::NaiveDateTime;
use rust_decimal::Decimal;

use crate::database::shared_models::tickets::TicketStatus;
use crate::database::DatabaseEntity;

pub struct TicketsDatabaseView {
    rows: Vec<TicketsDatabaseViewRow>,
}

impl DatabaseEntity for TicketsDatabaseView {
    type Row = TicketsDatabaseViewRow;
    const ENTITY_NAME: &str = "tickets_view";
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
pub struct TicketsDatabaseViewRow {
    pub id: i32,
    pub status: TicketStatus,
    pub customer: Option<String>,
    // pub device: String,
    pub balance: Decimal,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}
