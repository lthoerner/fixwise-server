use std::collections::HashSet;

use sqlx::query_builder::Separated;
use sqlx::Postgres;

use super::devices::DevicesDatabaseTable;
use super::parts::PartsDatabaseTable;
use super::tickets::TicketsDatabaseTable;
use super::IdentifiableRow;
use crate::database::loading_bar::LoadingBar;
use crate::database::{BulkInsert, DatabaseEntity};

pub struct BundledPartsDatabaseJunctionTable {
    rows: Vec<BundledPartsDatabaseJunctionTableRow>,
}

impl DatabaseEntity for BundledPartsDatabaseJunctionTable {
    type Row = BundledPartsDatabaseJunctionTableRow;
    const ENTITY_NAME: &str = "bundled_parts";
    const PRIMARY_COLUMN_NAME: &str = "(ticket, device, part)";

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

impl BulkInsert for BundledPartsDatabaseJunctionTable {
    const COLUMN_NAMES: &[&str] = &["ticket", "device", "part"];

    fn push_bindings(mut builder: Separated<Postgres, &str>, row: Self::Row) {
        builder
            .push_bind(row.ticket)
            .push_bind(row.device)
            .push_bind(row.part);
    }
}

#[derive(sqlx::FromRow, Clone)]
pub struct BundledPartsDatabaseJunctionTableRow {
    pub ticket: i32,
    pub device: i32,
    pub part: i32,
}

impl BundledPartsDatabaseJunctionTable {
    pub fn generate(
        count: usize,
        existing_tickets: &TicketsDatabaseTable,
        existing_devices: &DevicesDatabaseTable,
        existing_parts: &PartsDatabaseTable,
    ) -> Self {
        let mut rows = Vec::new();
        let mut existing_pairs = HashSet::new();
        let mut loading_bar = LoadingBar::new(count);
        for _ in 0..count {
            loading_bar.update();
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
