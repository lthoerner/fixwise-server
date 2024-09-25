use std::collections::HashSet;

use chrono::NaiveDateTime;

use proc_macros::{BulkInsert, DatabaseEntity, GenerateTableData, IdentifiableRow, SingleInsert};

use super::customers::CustomersDatabaseTable;
use super::generators::*;
use super::invoices::InvoicesDatabaseTable;
use super::ticket_devices::TicketDevicesDatabaseJunctionTable;
use super::IdentifiableRow;
use crate::database::shared_models::TicketStatus;
use crate::database::{DatabaseEntity, GenerateRowData};

#[derive(DatabaseEntity, BulkInsert, GenerateTableData, Clone)]
#[entity(
    entity_name = "tickets",
    primary_key = "id",
    foreign_key_name = "ticket",
    dependent_tables = [TicketDevicesDatabaseJunctionTable]
)]
pub struct TicketsDatabaseTable {
    rows: Vec<TicketsDatabaseTableRow>,
}

#[derive(SingleInsert, sqlx::FromRow, IdentifiableRow, Clone)]
pub struct TicketsDatabaseTableRow {
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

impl GenerateRowData for TicketsDatabaseTableRow {
    type Identifier = i32;
    type Dependencies<'a> = (&'a CustomersDatabaseTable, &'a InvoicesDatabaseTable);
    fn generate(
        _existing_rows: &[Self],
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
