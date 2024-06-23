use std::collections::HashSet;

use chrono::NaiveDateTime;
use rust_decimal::prelude::ToPrimitive;
use rust_decimal::Decimal;
use sqlx::query_builder::Separated;
use sqlx::Postgres;

use super::customers::CustomersDatabaseTable;
use super::generators::*;
use super::IdentifiableRow;
use crate::database::shared_models::tickets::TicketStatus;
use crate::database::{BulkInsert, DatabaseEntity};

pub struct TicketsDatabaseTable {
    rows: Vec<TicketsDatabaseTableRow>,
}

impl DatabaseEntity for TicketsDatabaseTable {
    type Row = TicketsDatabaseTableRow;
    const ENTITY_NAME: &str = "tickets";
    const PRIMARY_COLUMN_NAME: &str = "id";

    fn with_rows(rows: Vec<Self::Row>) -> Self {
        Self { rows }
    }

    fn take_rows(self) -> Vec<Self::Row> {
        self.rows
    }

    fn rows(&self) -> &[Self::Row] {
        &self.rows
    }
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
    pub customer: i32,
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

impl TicketsDatabaseTable {
    fn generate(count: usize, existing_customers: &CustomersDatabaseTable) -> Self {
        let mut rows = Vec::new();
        let mut existing_ids = HashSet::new();
        for _ in 0..count {
            rows.push(TicketsDatabaseTableRow::generate(
                &mut existing_ids,
                existing_customers,
            ))
        }

        Self::with_rows(rows)
    }
}

impl TicketsDatabaseTableRow {
    fn generate(
        existing_ids: &mut HashSet<i32>,
        existing_customers: &CustomersDatabaseTable,
    ) -> Self {
        let invoice_total = generate_dollar_value(Some(100.00), Some(1000.00));
        let payment_total = generate_dollar_value(None, Some(invoice_total.to_f32().unwrap()));
        let created_at = generate_date(None);
        let updated_at = generate_date(Some(created_at));

        Self {
            id: generate_unique_i32(0, existing_ids),
            status: generate_ticket_status(),
            customer: existing_customers.pick_random().id(),
            invoice_total,
            payment_total,
            description: generate_diagnostic(),
            notes: Vec::new(),
            created_at,
            updated_at,
        }
    }
}
