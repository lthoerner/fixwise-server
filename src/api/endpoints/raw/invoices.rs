use chrono::NaiveDateTime;
use rust_decimal::Decimal;
use serde::Serialize;

use proc_macros::{FromRecord, FromRelation, ServeEntityJson, ServeRowJson};

use crate::api::GenericIdParameter;
use crate::database::views::invoices::{InvoicesView, InvoicesViewRecord};
use crate::database::Relation;

#[derive(FromRelation, ServeEntityJson, Serialize)]
#[endpoint(relation = InvoicesView, raw = true)]
pub struct InvoicesApiEndpoint {
    rows: Vec<InvoicesApiEndpointRow>,
}

#[derive(FromRecord, ServeRowJson, Serialize)]
#[endpoint_row(id_param = GenericIdParameter, record = InvoicesViewRecord, raw = true)]
pub struct InvoicesApiEndpointRow {
    pub id: i32,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub invoice_total: Decimal,
    pub payment_total: Decimal,
}
