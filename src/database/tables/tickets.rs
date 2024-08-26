use std::collections::HashSet;

use chrono::NaiveDateTime;
use rust_decimal::prelude::ToPrimitive;
use rust_decimal::Decimal;
use sqlx::query_builder::Separated;
use sqlx::Postgres;

use proc_macros::DatabaseEntity;

use super::customers::CustomersDatabaseTable;
use super::generators::*;
use super::IdentifiableRow;
use crate::database::shared_models::tickets::TicketStatus;
use crate::database::{BulkInsert, DatabaseEntity, GenerateRowData, GenerateTableData};

#[derive(DatabaseEntity)]
#[entity(entity_name = "tickets", primary_column = "id")]
pub struct TicketsDatabaseTable {
    rows: Vec<TicketsDatabaseTableRow>,
}

impl BulkInsert for TicketsDatabaseTable {
    const COLUMN_NAMES: &[&str] = &[
        "id",
        "status",
        "customer",
        "invoice_total",
        "payment_total",
        "description",
        "notes",
        "created_at",
        "updated_at",
    ];

    fn push_bindings(mut builder: Separated<Postgres, &str>, row: Self::Row) {
        builder
            .push_bind(row.id)
            .push_bind(row.status)
            .push_bind(row.customer)
            .push_bind(row.invoice_total)
            .push_bind(row.payment_total)
            .push_bind(row.description)
            .push_bind(row.notes)
            .push_bind(row.created_at)
            .push_bind(row.updated_at);
    }
}

#[derive(sqlx::FromRow, Clone)]
pub struct TicketsDatabaseTableRow {
    pub id: i32,
    pub status: TicketStatus,
    pub customer: Option<i32>,
    pub invoice_total: Decimal,
    pub payment_total: Decimal,
    pub description: String,
    pub notes: Vec<String>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

impl IdentifiableRow for TicketsDatabaseTableRow {
    fn id(&self) -> i32 {
        self.id
    }
}

impl GenerateTableData for TicketsDatabaseTable {}
impl GenerateRowData for TicketsDatabaseTableRow {
    type Identifier = i32;
    type Dependencies<'a> = &'a CustomersDatabaseTable;
    fn generate(
        existing_ids: &mut HashSet<Self::Identifier>,
        dependencies: Self::Dependencies<'_>,
    ) -> Self {
        let invoice_total = generate_dollar_value(Some(100.00), Some(1000.00));
        let payment_total = generate_dollar_value(None, Some(invoice_total.to_f32().unwrap()));
        let created_at = generate_date(None);
        let updated_at = generate_date(Some(created_at));

        Self {
            id: generate_unique_i32(0, existing_ids),
            status: generate_ticket_status(),
            customer: generate_option(dependencies.pick_random().id(), 0.95),
            invoice_total,
            payment_total,
            description: generate_diagnostic(),
            notes: Vec::new(),
            created_at,
            updated_at,
        }
    }
}
