use std::collections::HashSet;

use rust_decimal::Decimal;

use proc_macros::{BulkInsert, DatabaseEntity, GenerateTableData, SingleInsert};

use super::devices::DevicesDatabaseTable;
use super::generators::*;
use super::tickets::TicketsDatabaseTable;
use super::IdentifiableRow;
use crate::database::{DatabaseEntity, GenerateRowData};

#[derive(DatabaseEntity, BulkInsert, GenerateTableData)]
#[entity(entity_name = "ticket_devices", primary_key = "(ticket, device)")]
pub struct TicketDevicesDatabaseJunctionTable {
    rows: Vec<TicketDevicesDatabaseJunctionTableRow>,
}

#[derive(SingleInsert, sqlx::FromRow, Clone)]
pub struct TicketDevicesDatabaseJunctionTableRow {
    pub ticket: i32,
    pub device: i32,
    pub diagnostic: Option<String>,
    // TODO: Probably refactor these to be NOT NULL with default 0
    pub labor_fee: Option<Decimal>,
}

impl GenerateRowData for TicketDevicesDatabaseJunctionTableRow {
    type Identifier = (i32, i32);
    type Dependencies<'a> = (&'a TicketsDatabaseTable, &'a DevicesDatabaseTable);
    fn generate(
        _existing_rows: &[Self],
        existing_pairs: &mut HashSet<Self::Identifier>,
        dependencies: Self::Dependencies<'_>,
    ) -> Self {
        let mut ticket_id = 0;
        let mut device_id = 0;
        let mut first_roll = true;
        while first_roll || existing_pairs.get(&(ticket_id, device_id)).is_some() {
            ticket_id = dependencies.0.pick_random().id();
            device_id = dependencies.1.pick_random().id();
            first_roll = false;
        }

        existing_pairs.insert((ticket_id, device_id));

        Self {
            ticket: ticket_id,
            device: device_id,
            diagnostic: generate_option(generate_diagnostic(), 0.6),
            labor_fee: generate_option(generate_dollar_value(Some(0.00), Some(200.00)), 0.7),
        }
    }
}
