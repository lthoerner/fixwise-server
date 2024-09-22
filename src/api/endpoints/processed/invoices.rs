use chrono::NaiveDateTime;
use rust_decimal::Decimal;
use serde::Serialize;

use proc_macros::{
    FromDatabaseEntity, FromDatabaseRow, ProcessEndpoint, ServeEntityJson, ServeRowJson,
};

use crate::api::endpoints::ViewCell;
use crate::api::GenericIdParameter;
use crate::database::views::invoices::{InvoicesDatabaseView, InvoicesDatabaseViewRow};
use crate::database::DatabaseEntity;

#[derive(FromDatabaseEntity, ServeEntityJson, Serialize)]
#[endpoint(database_entity = InvoicesDatabaseView, raw = false)]
pub struct InvoicesApiEndpoint {
    metadata: EndpointMetadata,
    rows: Vec<InvoicesApiEndpointRow>,
}

#[derive(ProcessEndpoint, FromDatabaseRow, ServeRowJson, Serialize)]
#[endpoint_row(id_param = GenericIdParameter, database_row = InvoicesDatabaseViewRow, raw = false)]
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
