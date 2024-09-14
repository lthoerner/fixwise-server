use chrono::NaiveDateTime;
use rust_decimal::Decimal;

use proc_macros::DatabaseEntity;

use crate::database::shared_models::TicketStatus;

#[derive(DatabaseEntity)]
#[entity(entity_name = "tickets_view", primary_key = "id")]
pub struct TicketsDatabaseView {
    rows: Vec<TicketsDatabaseViewRow>,
}

#[derive(sqlx::FromRow, Clone)]
pub struct TicketsDatabaseViewRow {
    pub id: i32,
    pub status: TicketStatus,
    pub customer: Option<String>,
    pub balance: Decimal,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}
