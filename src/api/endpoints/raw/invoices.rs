use chrono::NaiveDateTime;
use rust_decimal::Decimal;
use serde::Serialize;

use crate::api::{
    FromDatabaseEntity, FromDatabaseRow, GenericIdParameter, ServeEntityJson, ServeRowJson,
};
use crate::database::views::invoices::{InvoicesDatabaseView, InvoicesDatabaseViewRow};
use crate::database::DatabaseEntity;

#[derive(Serialize)]
pub struct InvoicesApiEndpoint {
    rows: Vec<InvoicesApiEndpointRow>,
}

#[derive(Serialize)]
pub struct InvoicesApiEndpointRow {
    pub id: i32,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub invoice_total: Decimal,
    pub payment_total: Decimal,
}

impl ServeEntityJson for InvoicesApiEndpoint {}
impl FromDatabaseEntity for InvoicesApiEndpoint {
    type Entity = InvoicesDatabaseView;
    fn from_database_entity(entity: Self::Entity) -> Self {
        Self {
            rows: entity
                .take_rows()
                .into_iter()
                .map(InvoicesApiEndpointRow::from_database_row)
                .collect(),
        }
    }
}

impl ServeRowJson<GenericIdParameter> for InvoicesApiEndpointRow {}
impl FromDatabaseRow for InvoicesApiEndpointRow {
    type Row = InvoicesDatabaseViewRow;
    fn from_database_row(row: Self::Row) -> Self {
        InvoicesApiEndpointRow {
            id: row.id,
            created_at: row.created_at,
            updated_at: row.updated_at,
            invoice_total: row.invoice_total,
            payment_total: row.payment_total,
        }
    }
}
