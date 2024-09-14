use std::collections::HashSet;

use chrono::NaiveDateTime;
use rust_decimal::prelude::ToPrimitive;
use rust_decimal::Decimal;

use proc_macros::{BulkInsert, DatabaseEntity, GenerateTableData, IdentifiableRow, SingleInsert};

use super::generators::*;
use super::invoice_items::InvoiceItemsDatabaseTable;
use super::invoices::InvoicesDatabaseTable;
use super::items::ItemsDatabaseTable;
use super::product_prices::ProductPricesDatabaseTable;
use super::service_prices::ServicePricesDatabaseTable;
use super::IdentifiableRow;
use crate::database::shared_models::PaymentType;
use crate::database::{DatabaseEntity, GenerateRowData};

#[derive(DatabaseEntity, BulkInsert, GenerateTableData, Clone)]
#[entity(entity_name = "invoice_payments", primary_key = "id")]
pub struct InvoicePaymentsDatabaseTable {
    rows: Vec<InvoicePaymentsDatabaseTableRow>,
}

#[derive(SingleInsert, sqlx::FromRow, IdentifiableRow, Clone)]
pub struct InvoicePaymentsDatabaseTableRow {
    pub id: i32,
    pub invoice: i32,
    pub amount: Decimal,
    #[sqlx(rename = "type")]
    pub r#type: PaymentType,
    #[defaultable]
    pub timestamp: Option<NaiveDateTime>,
}

impl GenerateRowData for InvoicePaymentsDatabaseTableRow {
    type Identifier = i32;
    type Dependencies<'a> = (
        &'a InvoicesDatabaseTable,
        &'a InvoiceItemsDatabaseTable,
        &'a ItemsDatabaseTable,
        &'a ProductPricesDatabaseTable,
        &'a ServicePricesDatabaseTable,
    );

    fn generate(
        existing_rows: &[Self],
        existing_ids: &mut HashSet<Self::Identifier>,
        dependencies: Self::Dependencies<'_>,
    ) -> Self {
        let (invoice, amount) = loop {
            let random_invoice = dependencies.0.pick_random();
            let invoice_total: Decimal = dependencies
                .1
                .rows()
                .iter()
                .filter(|i| i.invoice == random_invoice.id())
                .map(|i| {
                    dependencies
                        .2
                        .get_item_price_by_id(i.item, dependencies.3, dependencies.4)
                })
                .sum();
            let current_payment_total: Decimal = existing_rows
                .iter()
                .filter(|r| r.invoice == random_invoice.id())
                .map(|r| r.amount)
                .sum();

            let maximum_payment_amount = invoice_total - current_payment_total;
            if maximum_payment_amount < Decimal::new(1, 2) {
                continue;
            }

            break (
                random_invoice,
                generate_dollar_value(Some(0.01), Some(maximum_payment_amount.to_f32().unwrap())),
            );
        };

        Self {
            id: generate_unique_i32(0, existing_ids),
            invoice: invoice.id(),
            amount,
            r#type: generate_payment_type(),
            timestamp: Some(generate_date(invoice.created_at)),
        }
    }
}
