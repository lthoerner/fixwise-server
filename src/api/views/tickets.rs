use chrono::NaiveDateTime;
use rust_decimal::Decimal;
use serde::Serialize;

use super::{
    ColumnFormat, CssColor, FrontendColumnDisplay, FrontendColumnMetadata, FrontendDataType,
    TagOption, ViewCell,
};
use crate::api::FromDatabaseEntity;
use crate::database::shared_models::tickets::TicketStatus;
use crate::database::views::tickets::{TicketsDatabaseView, TicketsDatabaseViewRow};
use crate::database::DatabaseEntity;

#[derive(Serialize)]
pub struct TicketsApiView {
    metadata: TicketsApiViewMetadata,
    rows: Vec<TicketsApiViewRow>,
}

#[derive(Serialize)]
struct TicketsApiViewRow {
    id: ViewCell<u32>,
    status: ViewCell<TicketStatus>,
    customer_name: ViewCell<String>,
    device: ViewCell<String>,
    balance: ViewCell<Decimal>,
    created_at: ViewCell<NaiveDateTime>,
    updated_at: ViewCell<NaiveDateTime>,
}

struct TicketsApiViewFormatting {
    id: ColumnFormat,
    status: ColumnFormat,
    customer_name: ColumnFormat,
    device: ColumnFormat,
    balance: ColumnFormat,
    created_at: ColumnFormat,
    updated_at: ColumnFormat,
}

#[derive(Serialize)]
struct TicketsApiViewMetadata {
    id: FrontendColumnMetadata,
    status: FrontendColumnMetadata,
    customer_name: FrontendColumnMetadata,
    device: FrontendColumnMetadata,
    balance: FrontendColumnMetadata,
    created_at: FrontendColumnMetadata,
    updated_at: FrontendColumnMetadata,
}

impl TicketsApiViewFormatting {
    const fn new() -> Self {
        Self {
            id: ColumnFormat::Id,
            status: ColumnFormat::Tag,
            customer_name: ColumnFormat::None,
            device: ColumnFormat::None,
            balance: ColumnFormat::Currency,
            created_at: ColumnFormat::Date,
            updated_at: ColumnFormat::Date,
        }
    }
}

impl TicketsApiViewMetadata {
    const fn new() -> Self {
        Self {
            id: FrontendColumnMetadata {
                data_type: FrontendDataType::Integer,
                display: FrontendColumnDisplay::Text {
                    name: "ID",
                    trimmable: false,
                },
            },
            status: FrontendColumnMetadata {
                data_type: FrontendDataType::Tag,
                display: FrontendColumnDisplay::Tag {
                    name: "Status",
                    options: &[
                        TagOption {
                            name: "open",
                            color: CssColor::Preset("limegreen"),
                        },
                        TagOption {
                            name: "closed",
                            color: CssColor::Preset("red"),
                        },
                    ],
                },
            },
            customer_name: FrontendColumnMetadata {
                data_type: FrontendDataType::String,
                display: FrontendColumnDisplay::Text {
                    name: "Customer",
                    trimmable: true,
                },
            },
            device: FrontendColumnMetadata {
                data_type: FrontendDataType::String,
                display: FrontendColumnDisplay::Text {
                    name: "Device",
                    trimmable: true,
                },
            },
            balance: FrontendColumnMetadata {
                data_type: FrontendDataType::Decimal,
                display: FrontendColumnDisplay::Text {
                    name: "Balance",
                    trimmable: false,
                },
            },
            created_at: FrontendColumnMetadata {
                data_type: FrontendDataType::Timestamp,
                display: FrontendColumnDisplay::Text {
                    name: "Created",
                    trimmable: false,
                },
            },
            updated_at: FrontendColumnMetadata {
                data_type: FrontendDataType::Timestamp,
                display: FrontendColumnDisplay::Text {
                    name: "Updated",
                    trimmable: false,
                },
            },
        }
    }
}

impl FromDatabaseEntity for TicketsApiView {
    type Entity = TicketsDatabaseView;
    fn from_database_entity(entity: Self::Entity) -> Self {
        let formatting = TicketsApiViewFormatting::new();
        Self {
            metadata: TicketsApiViewMetadata::new(),
            rows: entity
                .rows()
                .into_iter()
                .map(|row| {
                    let TicketsDatabaseViewRow {
                        id,
                        status,
                        customer_name,
                        device,
                        balance,
                        created_at,
                        updated_at,
                    } = row;

                    TicketsApiViewRow {
                        id: ViewCell::new(id as u32, &formatting.id),
                        status: ViewCell::new(status, &formatting.status),
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
