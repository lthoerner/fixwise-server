use chrono::NaiveDateTime;
use rust_decimal::Decimal;
use serde::Serialize;

use crate::api::{
    FromDatabaseEntity, FromDatabaseRow, GenericIdParameter, ServeEntityJson, ServeRowJson,
};
use crate::database::views::invoices::{InvoicesDatabaseView, InvoicesDatabaseViewRow};
use crate::database::DatabaseEntity;

#[derive(Serialize)]
pub struct InvoicesApiView {
    rows: Vec<InvoicesApiViewRow>,
}

#[derive(Serialize)]
pub struct InvoicesApiViewRow {
    pub id: i32,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub invoice_total: Decimal,
    pub payment_total: Decimal,
}

impl ServeEntityJson for InvoicesApiView {}
impl FromDatabaseEntity for InvoicesApiView {
    type Entity = InvoicesDatabaseView;
    fn from_database_entity(entity: Self::Entity) -> Self {
        Self {
            rows: entity
                .take_rows()
                .into_iter()
                .map(InvoicesApiViewRow::from_database_row)
                .collect(),
        }
    }
}

impl ServeRowJson<GenericIdParameter> for InvoicesApiViewRow {}
impl FromDatabaseRow for InvoicesApiViewRow {
    type Row = InvoicesDatabaseViewRow;
    fn from_database_row(row: Self::Row) -> Self {
        InvoicesApiViewRow {
            id: row.id,
            created_at: row.created_at,
            updated_at: row.updated_at,
            invoice_total: row.invoice_total,
            payment_total: row.payment_total,
        }
    }
}
