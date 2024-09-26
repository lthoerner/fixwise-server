use chrono::NaiveDateTime;
use rust_decimal::Decimal;
use serde::Serialize;

use proc_macros::{FromRecord, FromRelation, ServeResourceJson, ServeRecordJson};

use crate::api::GenericIdParameter;
use crate::database::views::invoices::{InvoicesView, InvoicesViewRecord};
use crate::database::Relation;

#[derive(FromRelation, ServeResourceJson, Serialize)]
#[resource(relation = InvoicesView, raw = true)]
pub struct InvoicesResource {
    records: Vec<InvoicesResourceRecord>,
}

#[derive(FromRecord, ServeRecordJson, Serialize)]
#[resource_record(id_param = GenericIdParameter, record = InvoicesViewRecord, raw = true)]
pub struct InvoicesResourceRecord {
    pub id: i32,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub invoice_total: Decimal,
    pub payment_total: Decimal,
}
