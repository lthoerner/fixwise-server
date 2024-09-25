use chrono::NaiveDateTime;
use rust_decimal::Decimal;
use serde::Serialize;

use proc_macros::{FromRecord, FromRelation, ProcessEndpoint, ServeEntityJson, ServeRowJson};

use crate::api::endpoints::ViewCell;
use crate::api::GenericIdParameter;
use crate::database::views::invoices::{InvoicesView, InvoicesViewRecord};
use crate::database::Relation;

#[derive(FromRelation, ServeEntityJson, Serialize)]
#[endpoint(relation = InvoicesView, raw = false)]
pub struct InvoicesApiEndpoint {
    metadata: EndpointMetadata,
    rows: Vec<InvoicesApiEndpointRow>,
}

#[derive(ProcessEndpoint, FromRecord, ServeRowJson, Serialize)]
#[endpoint_row(id_param = GenericIdParameter, record = InvoicesViewRecord, raw = false)]
pub struct InvoicesApiEndpointRow {
    #[col_format(preset = "id")]
    id: ViewCell<i32>,
    #[col_format(preset = "date", display_name = "Created")]
    created_at: ViewCell<NaiveDateTime>,
    #[col_format(preset = "date", display_name = "Updated")]
    updated_at: ViewCell<NaiveDateTime>,
    #[col_format(preset = "currency")]
    invoice_total: ViewCell<Decimal>,
    #[col_format(preset = "currency")]
    payment_total: ViewCell<Decimal>,
}
