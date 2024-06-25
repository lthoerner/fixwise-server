use serde::Serialize;

#[derive(Debug, Clone, Serialize, sqlx::Type)]
#[serde(rename_all = "snake_case")]
#[sqlx(type_name = "ticket_status", rename_all = "snake_case")]
pub enum TicketStatus {
    New,
    WaitingForParts,
    WaitingForCustomer,
    InRepair,
    ReadyForPickup,
    Closed,
}
