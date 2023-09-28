#![allow(dead_code)]

use chrono::{DateTime, Utc};

use crate::customer::Customer;

struct PlaceholderType;

pub struct Ticket {
    status: TicketStatus,
    customer: Customer,
    items: Vec<TicketItem>,
    eta: DateTime<Utc>,
    in_possession: bool,
    creation_time: DateTime<Utc>,
    // TODO: Replace with full update history
    last_update_time: DateTime<Utc>,
    // TODO: `Note` struct with extra information
    notes: Vec<String>,
}

pub enum TicketStatus {
    InDiagnosis,
    InRepair,
    WaitingForPart,
    WaitingForContact,
    ReadyForPickup,
    Closed,
}

pub struct TicketItem {
    // TODO: Add support for other inventory types
    pub device: DeviceModel,
    pub repair: RepairKind,
    pub price: f32,
    // ? Should there be item-specific notes?
}

pub enum DeviceModel {
    Apple(ApplePhone),
    Samsung(SamsungPhone),
    Google(GooglePhone),
    Motorola(MotorolaPhone),
    // TODO: Support more devics (other models and device types)
    Other,
}

pub enum ApplePhone {}
pub enum SamsungPhone {}
pub enum GooglePhone {}
pub enum MotorolaPhone {}

pub enum RepairKind {
    Screen,
    Battery,
    Backglass,
    FrontCamera,
    RearCamera,
    ChargePort,
    Other,
}
