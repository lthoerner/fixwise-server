use std::collections::HashSet;

use crudkit::prelude::*;
use serde::Serialize;

use proc_macros::GenerateTable;

use super::invoices::InvoicesTable;
use super::items::ItemsTable;
use crate::database::traits::GenerateRecord;

#[derive(Relation, ReadRelation, WriteRelation, BulkInsert, GenerateTable, Serialize, Clone)]
#[relation(relation_name = "invoice_items", primary_key = "(invoice, item)")]
pub struct InvoiceItemsTable {
    records: Vec<InvoiceItemsTableRecord>,
}

#[derive(Record, ReadRecord, WriteRecord, SingleInsert, Serialize, sqlx::FromRow, Clone)]
pub struct InvoiceItemsTableRecord {
    #[manual_primary_key]
    pub invoice: i32,
    #[manual_primary_key]
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
            invoice = dependencies.0.pick_random().id().unwrap();
            item = dependencies.1.pick_random().id().unwrap();
            first_roll = false;
        }

        existing_pairs.insert((invoice, item));

        Self { invoice, item }
    }
}
