use std::collections::HashSet;

use proc_macros::{BulkInsert, DatabaseEntity, GenerateTableData, SingleInsert};

use super::devices::DevicesDatabaseTable;
use super::generators::*;
use super::services::ServicesDatabaseTable;
use super::tickets::TicketsDatabaseTable;
use super::IdentifiableRow;
use crate::database::{DatabaseEntity, GenerateRowData};

#[derive(DatabaseEntity, BulkInsert, GenerateTableData, Clone)]
#[entity(entity_name = "ticket_devices", primary_key = "(ticket, device)")]
pub struct TicketDevicesDatabaseJunctionTable {
    rows: Vec<TicketDevicesDatabaseJunctionTableRow>,
}

#[derive(SingleInsert, sqlx::FromRow, Clone)]
pub struct TicketDevicesDatabaseJunctionTableRow {
    pub ticket: i32,
    pub device: i32,
    pub service: i32,
    pub diagnostic: Option<String>,
}

impl GenerateRowData for TicketDevicesDatabaseJunctionTableRow {
    type Identifier = (i32, i32);
    type Dependencies<'a> = (
        &'a TicketsDatabaseTable,
        &'a DevicesDatabaseTable,
        &'a ServicesDatabaseTable,
    );

    fn generate(
        _existing_rows: &[Self],
        existing_pairs: &mut HashSet<Self::Identifier>,
        dependencies: Self::Dependencies<'_>,
    ) -> Self {
        let mut ticket = 0;
        let mut device = 0;
        let mut first_roll = true;
        while first_roll || existing_pairs.get(&(ticket, device)).is_some() {
            ticket = dependencies.0.pick_random().id();
            device = dependencies.1.pick_random().id();
            first_roll = false;
        }

        existing_pairs.insert((ticket, device));

        Self {
            ticket,
            device,
            service: dependencies.2.pick_random().id(),
            diagnostic: generate_option(generate_diagnostic(), 0.6),
        }
    }
}
