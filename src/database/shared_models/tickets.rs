use serde::Serialize;

#[derive(Debug, Clone, Serialize, sqlx::Type)]
#[sqlx(type_name = "ticket_status", rename_all = "lowercase")]
#[serde(rename_all = "lowercase")]
pub enum TicketStatus {
    Open,
    Closed,
}

impl ToString for TicketStatus {
    fn to_string(&self) -> String {
        match self {
            TicketStatus::Open => "Open".to_owned(),
            TicketStatus::Closed => "Closed".to_owned(),
        }
    }
}
