use std::collections::HashSet;

use rust_decimal::Decimal;
use sqlx::query_builder::Separated;
use sqlx::Postgres;

use super::devices::DevicesDatabaseTable;
use super::generators::*;
use super::tickets::TicketsDatabaseTable;
use super::IdentifiableRow;
use crate::database::DatabaseEntity;

pub struct TicketDevicesDatabaseJunctionTable {
    rows: Vec<TicketDevicesDatabaseJunctionTableRow>,
}

impl DatabaseEntity for TicketDevicesDatabaseJunctionTable {
    type Row = TicketDevicesDatabaseJunctionTableRow;
    const ENTITY_NAME: &str = "ticket_devices";
    const COLUMN_NAMES: &[&str] = &["ticket", "device", "diagnostic", "labor_fee"];
    const PRIMARY_COLUMN_NAME: &str = "(ticket, device)";

    fn with_rows(rows: Vec<Self::Row>) -> Self {
        Self { rows }
    }

    fn take_rows(self) -> Vec<Self::Row> {
        self.rows
    }

    fn rows(&self) -> &[Self::Row] {
        &self.rows
    }

    fn push_bindings(mut builder: Separated<Postgres, &str>, row: Self::Row) {
        builder
            .push_bind(row.ticket)
            .push_bind(row.device)
            .push_bind(row.diagnostic)
            .push_bind(row.labor_fee);
    }
}

#[derive(sqlx::FromRow, Clone)]
pub struct TicketDevicesDatabaseJunctionTableRow {
    pub ticket: i32,
    pub device: i32,
    pub diagnostic: Option<String>,
    // TODO: Probably refactor these to be NOT NULL with default 0
    pub labor_fee: Option<Decimal>,
}

impl TicketDevicesDatabaseJunctionTable {
    fn generate(
        count: usize,
        existing_devices: &DevicesDatabaseTable,
        existing_tickets: &TicketsDatabaseTable,
    ) -> Self {
        let mut rows = Vec::new();
        let mut existing_pairs = HashSet::new();
        for _ in 0..count {
            rows.push(TicketDevicesDatabaseJunctionTableRow::generate(
                &mut existing_pairs,
                existing_tickets,
                existing_devices,
            ))
        }

        Self::with_rows(rows)
    }
}

impl TicketDevicesDatabaseJunctionTableRow {
    fn generate(
        existing_pairs: &mut HashSet<(i32, i32)>,
        existing_tickets: &TicketsDatabaseTable,
        existing_devices: &DevicesDatabaseTable,
    ) -> Self {
        let mut ticket_id = 0;
        let mut device_id = 0;
        let mut first_roll = true;
        while first_roll || existing_pairs.get(&(ticket_id, device_id)).is_some() {
            ticket_id = existing_tickets.pick_random().id();
            device_id = existing_devices.pick_random().id();
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
