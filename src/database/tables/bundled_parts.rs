use std::collections::HashSet;

use super::devices::DevicesDatabaseTable;
use super::parts::PartsDatabaseTable;
use super::tickets::TicketsDatabaseTable;
use super::IdentifiableRow;
use crate::database::DatabaseEntity;

pub struct BundledPartsDatabaseJunctionTable {
    rows: Vec<BundledPartsDatabaseJunctionTableRow>,
}

impl DatabaseEntity for BundledPartsDatabaseJunctionTable {
    type Row = BundledPartsDatabaseJunctionTableRow;
    const ENTITY_NAME: &'static str = "bundled_parts";
    const PRIMARY_COLUMN_NAME: &'static str = "(ticket, device, part)";

    fn with_rows(rows: Vec<Self::Row>) -> Self {
        Self { rows }
    }

    fn rows(self) -> Vec<Self::Row> {
        self.rows
    }

    fn borrow_rows(&self) -> &[Self::Row] {
        &self.rows
    }
}

#[derive(sqlx::FromRow, Clone)]
pub struct BundledPartsDatabaseJunctionTableRow {
    pub ticket: i32,
    pub device: i32,
    pub part: i32,
}

impl BundledPartsDatabaseJunctionTable {
    fn generate(
        count: usize,
        existing_tickets: &TicketsDatabaseTable,
        existing_devices: &DevicesDatabaseTable,
        existing_parts: &PartsDatabaseTable,
    ) -> Self {
        let mut rows = Vec::new();
        let mut existing_pairs = HashSet::new();
        for _ in 0..count {
            rows.push(BundledPartsDatabaseJunctionTableRow::generate(
                &mut existing_pairs,
                existing_tickets,
                existing_devices,
                existing_parts,
            ))
        }

        Self::with_rows(rows)
    }
}

impl BundledPartsDatabaseJunctionTableRow {
    fn generate(
        existing_pairs: &mut HashSet<(i32, i32, i32)>,
        existing_tickets: &TicketsDatabaseTable,
        existing_devices: &DevicesDatabaseTable,
        existing_parts: &PartsDatabaseTable,
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
            ticket_id = existing_tickets.pick_random().id();
            device_id = existing_devices.pick_random().id();
            part_id = existing_parts.pick_random().id();
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
