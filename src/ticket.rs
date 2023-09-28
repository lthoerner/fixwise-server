#![allow(dead_code)]

use chrono::{DateTime, Utc};

use crate::customer::Customer;
use crate::inventory::DeviceModel;

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

pub enum RepairKind {
    Screen,
    Battery,
    Backglass,
    Frame,
    FrontCamera,
    RearCamera,
    LensCover,
    ChargePort,
    Other,
}
