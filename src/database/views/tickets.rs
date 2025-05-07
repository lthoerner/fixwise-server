use chrono::NaiveDateTime;
use crudkit::{ReadRecord, ReadRelation, Record, Relation};
use rust_decimal::Decimal;
use serde::Serialize;

use crate::database::shared_models::TicketStatus;

#[derive(Relation, ReadRelation, Serialize)]
#[relation(relation_name = "tickets_view", primary_key = "id")]
pub struct TicketsView {
    records: Vec<TicketsViewRecord>,
}

#[derive(Record, ReadRecord, sqlx::FromRow, Serialize, Clone)]
pub struct TicketsViewRecord {
    pub id: i32,
    pub status: TicketStatus,
    pub customer: Option<String>,
    pub balance: Decimal,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}
