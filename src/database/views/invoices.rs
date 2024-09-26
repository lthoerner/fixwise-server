use chrono::NaiveDateTime;
use rust_decimal::Decimal;

use proc_macros::Relation;

#[derive(Relation)]
#[relation(relation_name = "invoices_view", primary_key = "id")]
pub struct InvoicesView {
    records: Vec<InvoicesViewRecord>,
}

#[derive(sqlx::FromRow, Clone)]
pub struct InvoicesViewRecord {
    pub id: i32,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub invoice_total: Decimal,
    pub payment_total: Decimal,
}
