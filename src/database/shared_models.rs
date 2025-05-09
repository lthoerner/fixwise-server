use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize, sqlx::Type)]
#[serde(rename_all = "snake_case")]
#[sqlx(type_name = "ticket_status", rename_all = "snake_case")]
pub enum TicketStatus {
    Pending,
    WaitingForParts,
    WaitingForCustomer,
    InRepair,
    UnableToRepair,
    ReadyForPickup,
    Cancelled,
    Collected,
}

#[derive(Debug, Clone, Deserialize, PartialEq, Serialize, sqlx::Type)]
#[serde(rename_all = "snake_case")]
#[sqlx(type_name = "payment_type", rename_all = "snake_case")]
pub enum PaymentType {
    Card,
    Cash,
}

#[derive(Debug, Clone, Deserialize, PartialEq, Serialize, sqlx::Type)]
#[serde(rename_all = "snake_case")]
#[sqlx(type_name = "item_type", rename_all = "snake_case")]
pub enum ItemType {
    Product,
    Service,
}
