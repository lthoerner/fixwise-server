use std::collections::HashSet;

use chrono::NaiveDateTime;

use proc_macros::{BulkInsert, GenerateTableData, IdentifiableRecord, Relation, SingleInsert};

use super::customers::CustomersTable;
use super::generators::*;
use super::invoices::InvoicesTable;
use super::ticket_devices::TicketDevicesJunctionTable;
use super::IdentifiableRecord;
use crate::database::shared_models::TicketStatus;
use crate::database::{GenerateRecord, Relation};

#[derive(Relation, BulkInsert, GenerateTableData, Clone)]
#[relation(
    relation_name = "tickets",
    primary_key = "id",
    foreign_key_name = "ticket",
    dependent_tables = [TicketDevicesJunctionTable]
)]
pub struct TicketsTable {
    records: Vec<TicketsTableRecord>,
}

#[derive(SingleInsert, sqlx::FromRow, IdentifiableRecord, Clone)]
pub struct TicketsTableRecord {
    pub id: i32,
    #[defaultable]
    pub status: Option<TicketStatus>,
    pub customer: Option<i32>,
    pub invoice: i32,
    pub description: String,
    #[defaultable]
    pub notes: Option<Vec<String>>,
    #[defaultable]
    pub created_at: Option<NaiveDateTime>,
    #[defaultable]
    pub updated_at: Option<NaiveDateTime>,
}

impl GenerateRecord for TicketsTableRecord {
    type Identifier = i32;
    type Dependencies<'a> = (&'a CustomersTable, &'a InvoicesTable);
    fn generate(
        _existing_records: &[Self],
        existing_ids: &mut HashSet<Self::Identifier>,
        dependencies: Self::Dependencies<'_>,
    ) -> Self {
        let created_at = generate_date(None);
        let updated_at = generate_date(Some(created_at));

        Self {
            id: generate_unique_i32(0, existing_ids),
            status: Some(generate_ticket_status()),
            customer: generate_option(dependencies.0.pick_random().id(), 0.95),
            invoice: dependencies.1.pick_random().id(),
            description: generate_diagnostic(),
            notes: None,
            created_at: Some(created_at),
            updated_at: Some(updated_at),
        }
    }
}
