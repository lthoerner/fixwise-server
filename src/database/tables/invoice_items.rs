use std::collections::HashSet;

use proc_macros::{BulkInsert, GenerateTableData, IdentifiableRecord, Relation, SingleInsert};

use super::invoices::InvoicesTable;
use super::items::ItemsTable;
use super::IdentifiableRecord;
use crate::database::{GenerateRecord, Relation};

#[derive(Relation, BulkInsert, GenerateTableData, Clone)]
#[relation(
    relation_name = "invoice_items",
    primary_key = "(invoice, item)",
    foreign_key_name = "invoice_item"
)]
pub struct InvoiceItemsTable {
    records: Vec<InvoiceItemsTableRecord>,
}

#[derive(SingleInsert, sqlx::FromRow, IdentifiableRecord, Clone)]
pub struct InvoiceItemsTableRecord {
    pub invoice: i32,
    pub item: i32,
}

impl GenerateRecord for InvoiceItemsTableRecord {
    type Identifier = (i32, i32);
    type Dependencies<'a> = (&'a InvoicesTable, &'a ItemsTable);
    fn generate(
        _existing_records: &[Self],
        existing_pairs: &mut HashSet<Self::Identifier>,
        dependencies: Self::Dependencies<'_>,
    ) -> Self {
        let mut invoice = 0;
        let mut item = 0;
        let mut first_roll = true;
        while first_roll || existing_pairs.get(&(invoice, item)).is_some() {
            invoice = dependencies.0.pick_random().id();
            item = dependencies.1.pick_random().id();
            first_roll = false;
        }

        existing_pairs.insert((invoice, item));

        Self { invoice, item }
    }
}
