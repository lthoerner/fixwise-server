use chrono::NaiveDateTime;
use rust_decimal::Decimal;

use proc_macros::DatabaseEntity;

#[derive(DatabaseEntity)]
#[entity(
    entity_name = "invoices_view",
    primary_key = "id",
    foreign_key_name = "PLACEHOLDER"
)]
pub struct InvoicesDatabaseView {
    rows: Vec<InvoicesDatabaseViewRow>,
}

#[derive(sqlx::FromRow, Clone)]
pub struct InvoicesDatabaseViewRow {
    pub id: i32,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub invoice_total: Decimal,
    pub payment_total: Decimal,
}
