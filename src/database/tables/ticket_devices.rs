use std::collections::HashSet;

use proc_macros::{BulkInsert, GenerateTableData, Relation, SingleInsert, Table};

use super::devices::DevicesTable;
use super::generators::*;
use super::services::ServicesTable;
use super::tickets::TicketsTable;
use super::IdentifiableRecord;
use crate::database::{GenerateRecord, Relation};

#[derive(Relation, Table, BulkInsert, GenerateTableData, Clone)]
#[relation(relation_name = "ticket_devices", primary_key = "(ticket, device)")]
#[table(foreign_key_name = "ticket_device")]
pub struct TicketDevicesJunctionTable {
    records: Vec<TicketDevicesJunctionTableRecord>,
}

#[derive(SingleInsert, sqlx::FromRow, Clone)]
pub struct TicketDevicesJunctionTableRecord {
    pub ticket: i32,
    pub device: i32,
    pub service: i32,
    pub diagnostic: Option<String>,
}

impl GenerateRecord for TicketDevicesJunctionTableRecord {
    type Identifier = (i32, i32);
    type Dependencies<'a> = (&'a TicketsTable, &'a DevicesTable, &'a ServicesTable);

    fn generate(
        _existing_records: &[Self],
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
