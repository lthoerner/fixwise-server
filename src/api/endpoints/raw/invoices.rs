use chrono::NaiveDateTime;
use rust_decimal::Decimal;
use serde::Serialize;

use proc_macros::{FromDatabaseEntity, FromDatabaseRow, ServeEntityJson, ServeRowJson};

use crate::api::GenericIdParameter;
use crate::database::views::invoices::{InvoicesDatabaseView, InvoicesDatabaseViewRow};
use crate::database::DatabaseEntity;

#[derive(FromDatabaseEntity, ServeEntityJson, Serialize)]
#[endpoint(database_entity = InvoicesDatabaseView, raw = true)]
pub struct InvoicesApiEndpoint {
    rows: Vec<InvoicesApiEndpointRow>,
}

#[derive(FromDatabaseRow, ServeRowJson, Serialize)]
#[endpoint_row(id_param = GenericIdParameter, database_row = InvoicesDatabaseViewRow, raw = true)]
pub struct InvoicesApiEndpointRow {
    pub id: i32,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub invoice_total: Decimal,
    pub payment_total: Decimal,
}
