use std::collections::HashSet;

use proc_macros::{BulkInsert, DatabaseEntity, SingleInsert};

use super::parts::PartsDatabaseTable;
use super::ticket_devices::TicketDevicesDatabaseJunctionTable;
use super::IdentifiableRow;
use crate::database::{DatabaseEntity, GenerateRowData, GenerateTableData};

#[derive(DatabaseEntity, BulkInsert)]
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

impl GenerateTableData for BundledPartsDatabaseJunctionTable {}
impl GenerateRowData for BundledPartsDatabaseJunctionTableRow {
    type Identifier = (i32, i32, i32);
    type Dependencies<'a> = (
        &'a TicketDevicesDatabaseJunctionTable,
        &'a PartsDatabaseTable,
    );
    fn generate(
        existing_pairs: &mut HashSet<Self::Identifier>,
        dependencies: Self::Dependencies<'_>,
    ) -> Self {
        let mut ticket_id = 0;
        let mut device_id = 0;
        let mut part_id = 0;
        let mut first_roll = true;
        while first_roll
            || existing_pairs
                .get(&(ticket_id, device_id, part_id))
                .is_some()
        {
            let ticket_device = dependencies.0.pick_random();
            (ticket_id, device_id) = (ticket_device.ticket, ticket_device.device);
            part_id = dependencies.1.pick_random().id();
            first_roll = false;
        }

        existing_pairs.insert((ticket_id, device_id, part_id));

        Self {
            ticket: ticket_id,
            device: device_id,
            part: part_id,
        }
    }
}
