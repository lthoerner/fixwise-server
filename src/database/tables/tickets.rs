use std::collections::{HashMap, HashSet};

use chrono::{Duration, NaiveDate, NaiveDateTime, NaiveTime};
use rand::{thread_rng, Rng};
use rust_decimal::Decimal;

use super::{Generate, IdentifiableRow};
use crate::database::shared_models::tickets::TicketStatus;
use crate::database::DatabaseEntity;

pub struct TicketsDatabaseTable {
    rows: Vec<TicketsDatabaseTableRow>,
}

impl DatabaseEntity for TicketsDatabaseTable {
    type Row = TicketsDatabaseTableRow;
    const ENTITY_NAME: &'static str = "tickets";
    const PRIMARY_COLUMN_NAME: &'static str = "id";

    fn with_rows(rows: Vec<Self::Row>) -> Self {
        Self { rows }
    }

    fn rows(self) -> Vec<Self::Row> {
        self.rows
    }
}

#[derive(sqlx::FromRow)]
pub struct TicketsDatabaseTableRow {
    pub id: i32,
    pub status: TicketStatus,
    pub customer_id: i32,
    pub device: String,
    pub diagnostic: String,
    pub invoice_amount: Decimal,
    pub payment_amount: Decimal,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

impl IdentifiableRow for TicketsDatabaseTableRow {
    fn id(&self) -> i32 {
        self.id
    }
}

impl Generate for TicketsDatabaseTableRow {
    fn generate<'a>(
        existing: &mut HashSet<i32>,
        dependencies: &'a HashMap<&'static str, &'a [impl IdentifiableRow]>,
    ) -> Self {
        // TODO: Static names here may change
        let existing_customers = dependencies.get("customers").unwrap();

        let status = match thread_rng().gen_range(0..=5) {
            0 => TicketStatus::New,
            1 => TicketStatus::WaitingForParts,
            2 => TicketStatus::WaitingForCustomer,
            3 => TicketStatus::InRepair,
            4 => TicketStatus::ReadyForPickup,
            5 => TicketStatus::Closed,
            _ => unreachable!(),
        };
        let invoice_amount = Decimal::new(thread_rng().gen_range(10000..=99999), 2);
        let payment_amount = invoice_amount / Decimal::new(thread_rng().gen_range(1..=10), 0);
        let created_at = Self::generate_date(None);
        let updated_at = Self::generate_date(Some(created_at));

        Self {
            id: crate::generate_unique_random_i32(0, existing),
            status,
            customer_id: existing_customers[thread_rng().gen_range(0..existing_customers.len())]
                .id(),
            device: Self::generate_device_name(),
            diagnostic: Self::generate_diagnostic(),
            invoice_amount,
            payment_amount,
            created_at,
            updated_at,
        }
    }
}

impl TicketsDatabaseTableRow {
    fn generate_device_name() -> String {
        const PHONE_LINES: [&str; 8] = [
            "iPhone",
            "Samsung Galaxy",
            "Google Pixel",
            "Motorola G",
            "LG",
            "Nokia",
            "Sony Xperia",
            "OnePlus",
        ];

        const MODIFIERS: [&str; 8] = ["Pro", "Max", "Ultra", "Plus", "Lite", "Mini", "X", "Z"];

        let phone = PHONE_LINES[thread_rng().gen_range(0..PHONE_LINES.len())];
        let generation = thread_rng().gen_range(1..=50);
        let modifier = MODIFIERS[thread_rng().gen_range(0..MODIFIERS.len())];

        format!("{} {} {}", phone, generation, modifier)
    }

    fn generate_diagnostic() -> String {
        const DIAGNOSTICS: [&str; 8] = [
            "Cracked Screen",
            "Battery Replacement",
            "Water Damage",
            "Charging Port",
            "Software Issue",
            "Speaker Issue",
            "Microphone Issue",
            "Camera Issue",
        ];

        DIAGNOSTICS[thread_rng().gen_range(0..DIAGNOSTICS.len())].to_owned()
    }

    fn generate_date(start: Option<NaiveDateTime>) -> NaiveDateTime {
        let start = start.unwrap_or_else(|| {
            NaiveDate::from_ymd_opt(2020, 1, 1)
                .unwrap()
                .and_hms_opt(0, 0, 0)
                .unwrap()
        });
        let end = NaiveDate::from_ymd_opt(2025, 1, 1)
            .unwrap()
            .and_hms_opt(0, 0, 0)
            .unwrap();
        let days_in_range = (end - start).num_days();
        let random_days = thread_rng().gen_range(0..=days_in_range);

        let hours = thread_rng().gen_range(0..24);
        let minutes = thread_rng().gen_range(0..60);
        let seconds = thread_rng().gen_range(0..60);

        NaiveDateTime::new(
            start.date() + Duration::days(random_days),
            NaiveTime::from_hms_opt(hours, minutes, seconds).unwrap(),
        )
    }
}
