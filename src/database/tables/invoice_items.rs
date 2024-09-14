use std::collections::HashSet;

use proc_macros::{BulkInsert, DatabaseEntity, GenerateTableData, IdentifiableRow, SingleInsert};

use super::invoices::InvoicesDatabaseTable;
use super::items::ItemsDatabaseTable;
use super::IdentifiableRow;
use crate::database::{DatabaseEntity, GenerateRowData};

#[derive(DatabaseEntity, BulkInsert, GenerateTableData, Clone)]
#[entity(entity_name = "invoice_items", primary_key = "(invoice, item)")]
pub struct InvoiceItemsDatabaseTable {
    rows: Vec<InvoiceItemsDatabaseTableRow>,
}

#[derive(SingleInsert, sqlx::FromRow, IdentifiableRow, Clone)]
pub struct InvoiceItemsDatabaseTableRow {
    pub invoice: i32,
    pub item: i32,
}

impl GenerateRowData for InvoiceItemsDatabaseTableRow {
    type Identifier = (i32, i32);
    type Dependencies<'a> = (&'a InvoicesDatabaseTable, &'a ItemsDatabaseTable);
    fn generate(
        _existing_rows: &[Self],
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
