use chrono::NaiveDateTime;
use rust_decimal::Decimal;
use serde::Serialize;

use super::{ColumnFormat, ViewCell};
use crate::api::FromDatabaseRow;
use crate::database::views::tickets::Ticket as DatabaseTicket;

#[derive(Serialize)]
pub struct Ticket {
    id: ViewCell<u32>,
    customer_name: ViewCell<String>,
    device: ViewCell<String>,
    balance: ViewCell<Decimal>,
    created_at: ViewCell<NaiveDateTime>,
    updated_at: ViewCell<NaiveDateTime>,
}

struct TicketFormatting {
    id: ColumnFormat,
    customer_name: ColumnFormat,
    device: ColumnFormat,
    balance: ColumnFormat,
    created_at: ColumnFormat,
    updated_at: ColumnFormat,
}

impl TicketFormatting {
    const fn new() -> Self {
        Self {
            id: ColumnFormat::Id,
            customer_name: ColumnFormat::None,
            device: ColumnFormat::None,
            balance: ColumnFormat::Currency,
            created_at: ColumnFormat::Date,
            updated_at: ColumnFormat::Date,
        }
    }
}

impl FromDatabaseRow for Ticket {
    type Entity = DatabaseTicket;
    fn from_database_row(row: Self::Entity) -> Self {
        let formatting = TicketFormatting::new();
        let DatabaseTicket {
            id,
            customer_name,
            device,
            balance,
            created_at,
            updated_at,
        } = row;

        Self {
            id: ViewCell::new(id as u32, formatting.id),
            customer_name: ViewCell::new(customer_name, formatting.customer_name),
            device: ViewCell::new(device, formatting.device),
            balance: ViewCell::new(balance, formatting.balance),
            created_at: ViewCell::new(created_at, formatting.created_at),
            updated_at: ViewCell::new(updated_at, formatting.updated_at),
        }
    }
}
