use chrono::NaiveDateTime;
use rust_decimal::Decimal;
use serde::Serialize;

use proc_macros::{ReadRecord, ReadRelation, Record, Relation};

#[derive(Relation, ReadRelation, Serialize)]
#[relation(relation_name = "invoices_view", primary_key = "id")]
pub struct InvoicesView {
    records: Vec<InvoicesViewRecord>,
}

#[derive(Record, ReadRecord, sqlx::FromRow, Serialize, Clone)]
pub struct InvoicesViewRecord {
    pub id: i32,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub invoice_total: Decimal,
    pub payment_total: Decimal,
}
