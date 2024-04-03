use chrono::NaiveDateTime;
use rust_decimal::Decimal;
use sqlx::FromRow;

use crate::database::DatabaseEntity;

#[derive(FromRow)]
pub struct TicketsDatabaseViewRow {
    pub id: i32,
    pub customer_name: String,
    pub device: String,
    pub balance: Decimal,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

impl DatabaseEntity for TicketsDatabaseViewRow {
    const ENTITY_NAME: &'static str = "tickets_view";
    const PRIMARY_COLUMN_NAME: &'static str = "id";
}
