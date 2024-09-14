use std::collections::HashSet;

use chrono::NaiveDateTime;
use rust_decimal::prelude::ToPrimitive;
use rust_decimal::Decimal;

use proc_macros::{BulkInsert, DatabaseEntity, GenerateTableData, IdentifiableRow, SingleInsert};

use super::customers::CustomersDatabaseTable;
use super::generators::*;
use super::IdentifiableRow;
use crate::database::shared_models::TicketStatus;
use crate::database::{DatabaseEntity, GenerateRowData};

#[derive(DatabaseEntity, BulkInsert, GenerateTableData, Clone)]
#[entity(entity_name = "tickets", primary_key = "id")]
pub struct TicketsDatabaseTable {
    rows: Vec<TicketsDatabaseTableRow>,
}

#[derive(SingleInsert, sqlx::FromRow, IdentifiableRow, Clone)]
pub struct TicketsDatabaseTableRow {
    pub id: i32,
    #[defaultable]
    pub status: Option<TicketStatus>,
    pub customer: Option<i32>,
    #[defaultable]
    pub invoice_total: Option<Decimal>,
    #[defaultable]
    pub payment_total: Option<Decimal>,
    pub description: String,
    #[defaultable]
    pub notes: Option<Vec<String>>,
    #[defaultable]
    pub created_at: Option<NaiveDateTime>,
    #[defaultable]
    pub updated_at: Option<NaiveDateTime>,
}

impl GenerateRowData for TicketsDatabaseTableRow {
    type Identifier = i32;
    type Dependencies<'a> = &'a CustomersDatabaseTable;
    fn generate(
        _existing_rows: &[Self],
        existing_ids: &mut HashSet<Self::Identifier>,
        dependencies: Self::Dependencies<'_>,
    ) -> Self {
        let invoice_total = generate_dollar_value(Some(100.00), Some(1000.00));
        let payment_total = generate_dollar_value(None, Some(invoice_total.to_f32().unwrap()));
        let created_at = generate_date(None);
        let updated_at = generate_date(Some(created_at));

        Self {
            id: generate_unique_i32(0, existing_ids),
            status: Some(generate_ticket_status()),
            customer: generate_option(dependencies.pick_random().id(), 0.95),
            invoice_total: Some(invoice_total),
            payment_total: Some(payment_total),
            description: generate_diagnostic(),
            notes: None,
            created_at: Some(created_at),
            updated_at: Some(updated_at),
        }
    }
}
