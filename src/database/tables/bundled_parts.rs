use std::collections::HashSet;

use sqlx::query_builder::Separated;
use sqlx::Postgres;

use super::parts::PartsDatabaseTable;
use super::ticket_devices::TicketDevicesDatabaseJunctionTable;
use super::IdentifiableRow;
use crate::database::{BulkInsert, DatabaseEntity, GenerateRowData, GenerateTableData};

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
