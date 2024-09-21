use chrono::NaiveDateTime;
use rust_decimal::Decimal;
use serde::Serialize;

use proc_macros::{FromDatabaseEntity, ProcessEndpoint, ServeEntityJson, ServeRowJson};

use crate::api::endpoints::ViewCell;
use crate::api::{FromDatabaseRow, GenericIdParameter};
use crate::database::views::invoices::{InvoicesDatabaseView, InvoicesDatabaseViewRow};
use crate::database::DatabaseEntity;

#[derive(FromDatabaseEntity, ServeEntityJson, Serialize)]
#[database_entity(InvoicesDatabaseView)]
pub struct InvoicesApiEndpoint {
    metadata: EndpointMetadata,
    rows: Vec<InvoicesApiEndpointRow>,
}

#[derive(ProcessEndpoint, ServeRowJson, Serialize)]
#[id_param(GenericIdParameter)]
pub struct InvoicesApiEndpointRow {
    #[col_format(preset = "id")]
    id: ViewCell<u32>,
    #[col_format(preset = "date", display_name = "Created")]
    created_at: ViewCell<NaiveDateTime>,
    #[col_format(preset = "date", display_name = "Updated")]
    updated_at: ViewCell<NaiveDateTime>,
    #[col_format(preset = "currency")]
    invoice_total: ViewCell<Decimal>,
    #[col_format(preset = "currency")]
    payment_total: ViewCell<Decimal>,
}

impl FromDatabaseRow for InvoicesApiEndpointRow {
    type Row = InvoicesDatabaseViewRow;
    fn from_database_row(row: Self::Row) -> Self {
        let formatting = EndpointFormatting::new();

        let InvoicesDatabaseViewRow {
            id,
            created_at,
            updated_at,
            invoice_total,
            payment_total,
        } = row;

        InvoicesApiEndpointRow {
            id: ViewCell::new(id as u32, &formatting.id),
            created_at: ViewCell::new(created_at, &formatting.created_at),
            updated_at: ViewCell::new(updated_at, &formatting.updated_at),
            invoice_total: ViewCell::new(invoice_total, &formatting.invoice_total),
            payment_total: ViewCell::new(payment_total, &formatting.payment_total),
        }
    }
}
