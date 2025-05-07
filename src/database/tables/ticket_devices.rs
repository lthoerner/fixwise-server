use std::collections::HashSet;

use crudkit::prelude::*;
use serde::Serialize;

use proc_macros::GenerateTable;

use super::devices::DevicesTable;
use super::generators::*;
use super::services::ServicesTable;
use super::tickets::TicketsTable;
use crate::database::traits::GenerateRecord;

#[derive(Relation, ReadRelation, WriteRelation, BulkInsert, GenerateTable, Serialize, Clone)]
#[relation(relation_name = "ticket_devices", primary_key = "(ticket, device)")]
pub struct TicketDevicesJunctionTable {
    records: Vec<TicketDevicesJunctionTableRecord>,
}

#[derive(Record, ReadRecord, WriteRecord, SingleInsert, Serialize, sqlx::FromRow, Clone)]
pub struct TicketDevicesJunctionTableRecord {
    #[manual_primary_key]
    pub ticket: i32,
    #[manual_primary_key]
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
            ticket = dependencies.0.pick_random().id().unwrap();
            device = dependencies.1.pick_random().id().unwrap();
            first_roll = false;
        }

        existing_pairs.insert((ticket, device));

        Self {
            ticket,
            device,
            service: dependencies.2.pick_random().id().unwrap(),
            diagnostic: generate_option(generate_diagnostic(), 0.6),
        }
    }
}
