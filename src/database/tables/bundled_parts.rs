use std::collections::HashSet;

use proc_macros::{BulkInsert, DatabaseEntity, GenerateTableData, SingleInsert};

use super::parts::PartsDatabaseTable;
use super::ticket_devices::TicketDevicesDatabaseJunctionTable;
use super::IdentifiableRow;
use crate::database::{DatabaseEntity, GenerateRowData};

#[derive(DatabaseEntity, BulkInsert, GenerateTableData, Clone)]
#[entity(entity_name = "bundled_parts", primary_key = "(ticket, device, part)")]
pub struct BundledPartsDatabaseJunctionTable {
    rows: Vec<BundledPartsDatabaseJunctionTableRow>,
}

#[derive(SingleInsert, sqlx::FromRow, Clone)]
pub struct BundledPartsDatabaseJunctionTableRow {
    pub ticket: i32,
    pub device: i32,
    pub part: i32,
}

impl GenerateRowData for BundledPartsDatabaseJunctionTableRow {
    type Identifier = (i32, i32, i32);
    type Dependencies<'a> = (
        &'a TicketDevicesDatabaseJunctionTable,
        &'a PartsDatabaseTable,
    );

    fn generate(
        _existing_rows: &[Self],
        existing_pairs: &mut HashSet<Self::Identifier>,
        dependencies: Self::Dependencies<'_>,
    ) -> Self {
        let mut ticket = 0;
        let mut device = 0;
        let mut part = 0;
        let mut first_roll = true;
        while first_roll || existing_pairs.get(&(ticket, device, part)).is_some() {
            let ticket_device = dependencies.0.pick_random();
            (ticket, device) = (ticket_device.ticket, ticket_device.device);
            part = dependencies.1.pick_random().id();
            first_roll = false;
        }

        existing_pairs.insert((ticket, device, part));

        Self {
            ticket,
            device,
            part,
        }
    }
}
