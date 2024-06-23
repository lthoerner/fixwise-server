pub mod bundled_parts;
pub mod compatible_parts;
pub mod customers;
pub mod device_categories;
pub mod device_manufacturers;
pub mod device_models;
pub mod devices;
pub mod part_categories;
pub mod part_manufacturers;
pub mod parts;
pub mod ticket_devices;
pub mod tickets;
pub mod vendors;

use std::collections::HashSet;

use chrono::{Duration, NaiveDate, NaiveDateTime, NaiveTime};
use fake::faker::address::en::{CityName, StateAbbr, StreetName, StreetSuffix};
use fake::faker::internet::en::FreeEmail;
use fake::faker::name::en::Name;
use fake::faker::phone_number::en::PhoneNumber;
use fake::Fake;
use rand::{thread_rng, Rng};
use rust_decimal::Decimal;

use super::shared_models::tickets::TicketStatus;

pub trait IdentifiableRow {
    fn id(&self) -> i32;
}

fn generate_option<T>(maybe_value: T, some_chance: f64) -> Option<T> {
    match thread_rng().gen_bool(some_chance) {
        true => Some(maybe_value),
        false => None,
    }
}

fn generate_unique_i32(min: i32, existing: &mut HashSet<i32>) -> i32 {
    let mut val = 0;
    let mut first_roll = true;
    while first_roll || existing.get(&val).is_some() {
        val = thread_rng().gen_range(min..=i32::MAX);
        first_roll = false;
    }

    existing.insert(val);

    val
}

fn generate_dollar_value(min: Option<f32>, max: Option<f32>) -> Decimal {
    let adjusted_min = (min.unwrap_or_default() * 100.0) as i64;
    let adjusted_max = (max.unwrap_or_default() * 100.0) as i64;
    Decimal::new(thread_rng().gen_range(adjusted_min..=adjusted_max), 2)
}

fn generate_name() -> String {
    Name().fake()
}

fn generate_email_address() -> String {
    FreeEmail().fake()
}

fn generate_phone_number() -> String {
    PhoneNumber().fake()
}

fn generate_address() -> String {
    format!(
        "{} {} {}, {}, {} {}",
        thread_rng().gen_range(1..=9999),
        StreetName().fake::<String>(),
        StreetSuffix().fake::<String>(),
        CityName().fake::<String>(),
        StateAbbr().fake::<String>(),
        thread_rng().gen_range(10000..=99999)
    )
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

fn generate_ticket_status() -> TicketStatus {
    match thread_rng().gen_range(0..=5) {
        0 => TicketStatus::New,
        1 => TicketStatus::WaitingForParts,
        2 => TicketStatus::WaitingForCustomer,
        3 => TicketStatus::InRepair,
        4 => TicketStatus::ReadyForPickup,
        5 => TicketStatus::Closed,
        _ => unreachable!(),
    }
}
