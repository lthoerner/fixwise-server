use std::collections::HashSet;

use chrono::NaiveDateTime;
use rust_decimal::prelude::ToPrimitive;
use rust_decimal::Decimal;

use proc_macros::{BulkInsert, GenerateTableData, IdentifiableRecord, Relation, SingleInsert};

use super::generators::*;
use super::invoice_items::InvoiceItemsTable;
use super::invoices::InvoicesTable;
use super::items::ItemsTable;
use super::product_prices::ProductPricesTable;
use super::service_prices::ServicePricesTable;
use super::IdentifiableRecord;
use crate::database::shared_models::PaymentType;
use crate::database::{GenerateRecord, Relation};

#[derive(Relation, BulkInsert, GenerateTableData, Clone)]
#[relation(
    relation_name = "invoice_payments",
    primary_key = "id",
    foreign_key_name = "invoice_payment"
)]
pub struct InvoicePaymentsTable {
    records: Vec<InvoicePaymentsTableRecord>,
}

#[derive(SingleInsert, sqlx::FromRow, IdentifiableRecord, Clone)]
pub struct InvoicePaymentsTableRecord {
    pub id: i32,
    pub invoice: i32,
    pub amount: Decimal,
    #[sqlx(rename = "type")]
    pub r#type: PaymentType,
    #[defaultable]
    pub timestamp: Option<NaiveDateTime>,
}

impl GenerateRecord for InvoicePaymentsTableRecord {
    type Identifier = i32;
    type Dependencies<'a> = (
        &'a InvoicesTable,
        &'a InvoiceItemsTable,
        &'a ItemsTable,
        &'a ProductPricesTable,
        &'a ServicePricesTable,
    );

    fn generate(
        existing_records: &[Self],
        existing_ids: &mut HashSet<Self::Identifier>,
        dependencies: Self::Dependencies<'_>,
    ) -> Self {
        let (invoice, amount) = loop {
            let random_invoice = dependencies.0.pick_random();
            let invoice_total: Decimal = dependencies
                .1
                .records()
                .iter()
                .filter(|i| i.invoice == random_invoice.id())
                .map(|i| {
                    dependencies.2.get_item_price_by_id(
                        i.item,
                        dependencies.3,
                        dependencies.4,
                        random_invoice.created_at.unwrap(),
                    )
                })
                .sum();
            let current_payment_total: Decimal = existing_records
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
