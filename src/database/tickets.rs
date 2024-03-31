use std::collections::HashSet;

use axum::Json;
use chrono::NaiveTime;
use chrono::{Duration, NaiveDate, NaiveDateTime};
use rand::thread_rng;
use rand::Rng;
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use sqlx::{query, Row};

use super::customers::Customer;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Ticket {
    pub id: i32,
    pub customer_id: i32,
    pub device: String,
    pub diagnostic: String,
    pub invoice_amount: Decimal,
    pub payment_amount: Decimal,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TicketView {
    pub id: i32,
    pub customer_name: String,
    pub device: String,
    pub balance: Decimal,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

impl Ticket {
    pub fn generate(existing: &mut HashSet<i32>, existing_customers: &[Customer]) -> Self {
        let id = crate::generate_unique_random_i32(0, existing);
        let customer_id =
            existing_customers[thread_rng().gen_range(0..existing_customers.len())].id;
        let device = Self::generate_device_name();
        let diagnostic = Self::generate_diagnostic();
        let invoice_amount = Decimal::new(thread_rng().gen_range(10000..=99999), 2);
        let payment_amount = invoice_amount / Decimal::new(thread_rng().gen_range(1..=10), 0);
        let created_at = Self::generate_date(None);
        let updated_at = Self::generate_date(Some(created_at));

        Self {
            id,
            customer_id,
            device,
            diagnostic,
            invoice_amount,
            payment_amount,
            created_at,
            updated_at,
        }
    }

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

pub async fn get_tickets() -> Json<Vec<TicketView>> {
    let ticket_rows = query("SELECT * FROM test.tickets_view ORDER BY id")
        .fetch_all(crate::get_db!())
        .await
        .unwrap();

    let mut tickets = Vec::new();
    for ticket in ticket_rows {
        tickets.push(TicketView {
            id: ticket.get("id"),
            customer_name: ticket.get("customer_name"),
            device: ticket.get("device"),
            balance: ticket.get("balance"),
            created_at: ticket.get("created_at"),
            updated_at: ticket.get("updated_at"),
        });
    }

    Json(tickets)
}
