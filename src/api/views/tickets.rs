use chrono::NaiveDateTime;
use rust_decimal::Decimal;
use serde::Serialize;

use super::{ColumnFormat, ViewCell};
use crate::api::FromDatabaseEntity;
use crate::database::views::tickets::{TicketsDatabaseView, TicketsDatabaseViewRow};
use crate::database::DatabaseEntity;

#[derive(Serialize)]
pub struct TicketsApiView {
    rows: Vec<TicketsApiViewRow>,
}

#[derive(Serialize)]
struct TicketsApiViewRow {
    id: ViewCell<u32>,
    customer_name: ViewCell<String>,
    device: ViewCell<String>,
    balance: ViewCell<Decimal>,
    created_at: ViewCell<NaiveDateTime>,
    updated_at: ViewCell<NaiveDateTime>,
}

struct TicketsFormatting {
    id: ColumnFormat,
    customer_name: ColumnFormat,
    device: ColumnFormat,
    balance: ColumnFormat,
    created_at: ColumnFormat,
    updated_at: ColumnFormat,
}

impl TicketsFormatting {
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

impl FromDatabaseEntity for TicketsApiView {
    type Entity = TicketsDatabaseView;
    fn from_database_entity(entity: Self::Entity) -> Self {
        let formatting = TicketsFormatting::new();
        Self {
            rows: entity
                .rows()
                .into_iter()
                .map(|row| {
                    let TicketsDatabaseViewRow {
                        id,
                        customer_name,
                        device,
                        balance,
                        created_at,
                        updated_at,
                    } = row;

                    TicketsApiViewRow {
                        id: ViewCell::new(id as u32, &formatting.id),
                        customer_name: ViewCell::new(customer_name, &formatting.customer_name),
                        device: ViewCell::new(device, &formatting.device),
                        balance: ViewCell::new(balance, &formatting.balance),
                        created_at: ViewCell::new(created_at, &formatting.created_at),
                        updated_at: ViewCell::new(updated_at, &formatting.updated_at),
                    }
                })
                .collect(),
        }
    }
}