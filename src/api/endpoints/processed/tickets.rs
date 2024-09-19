use chrono::NaiveDateTime;
use rust_decimal::Decimal;
use serde::Serialize;

use proc_macros::{ProcessEndpoint, ServeEntityJson, ServeRowJson};

use crate::api::endpoints::{CssColor, TagOption, ViewCell};
use crate::api::{FromDatabaseEntity, FromDatabaseRow, GenericIdParameter};
use crate::database::shared_models::TicketStatus;
use crate::database::views::tickets::{TicketsDatabaseView, TicketsDatabaseViewRow};
use crate::database::DatabaseEntity;

const STATUS_TAG_OPTIONS: &[TagOption] = &[
    TagOption {
        name: "new",
        color: CssColor::Preset {
            name: "royalblue",
            opacity: 0.45,
        },
    },
    TagOption {
        name: "waiting_for_parts",
        color: CssColor::Preset {
            name: "red",
            opacity: 0.37,
        },
    },
    TagOption {
        name: "waiting_for_customer",
        color: CssColor::Preset {
            name: "yellow",
            opacity: 0.43,
        },
    },
    TagOption {
        name: "in_repair",
        color: CssColor::Preset {
            name: "orange",
            opacity: 0.54,
        },
    },
    TagOption {
        name: "ready_for_pickup",
        color: CssColor::Preset {
            name: "limegreen",
            opacity: 0.37,
        },
    },
    TagOption {
        name: "closed",
        color: CssColor::Preset {
            name: "gray",
            opacity: 0.45,
        },
    },
];

#[derive(ServeEntityJson, Serialize)]
pub struct TicketsApiEndpoint {
    metadata: EndpointMetadata,
    rows: Vec<TicketsApiEndpointRow>,
}

#[derive(ProcessEndpoint, ServeRowJson, Serialize)]
#[id_param(GenericIdParameter)]
pub struct TicketsApiEndpointRow {
    #[col_format(
        format = "id",
        data_type = "integer",
        display_name = "ID",
        trimmable = false
    )]
    id: ViewCell<u32>,
    #[col_format(format = "tag", data_type = "tag", tag_options = STATUS_TAG_OPTIONS)]
    status: ViewCell<TicketStatus>,
    #[col_format(data_type = "string", trimmable = true)]
    customer: ViewCell<Option<String>>,
    #[col_format(format = "currency", data_type = "string", trimmable = false)]
    balance: ViewCell<Decimal>,
    #[col_format(
        format = "date",
        data_type = "timestamp",
        display_name = "Created",
        trimmable = false
    )]
    created_at: ViewCell<NaiveDateTime>,
    #[col_format(
        format = "date",
        data_type = "timestamp",
        display_name = "Updated",
        trimmable = false
    )]
    updated_at: ViewCell<NaiveDateTime>,
}

impl FromDatabaseEntity for TicketsApiEndpoint {
    type Entity = TicketsDatabaseView;
    fn from_database_entity(entity: Self::Entity) -> Self {
        Self {
            metadata: EndpointMetadata::new(),
            rows: entity
                .take_rows()
                .into_iter()
                .map(TicketsApiEndpointRow::from_database_row)
                .collect(),
        }
    }
}

impl FromDatabaseRow for TicketsApiEndpointRow {
    type Row = TicketsDatabaseViewRow;
    fn from_database_row(row: Self::Row) -> Self {
        let formatting = EndpointFormatting::new();

        let TicketsDatabaseViewRow {
            id,
            status,
            customer,
            balance,
            created_at,
            updated_at,
        } = row;

        TicketsApiEndpointRow {
            id: ViewCell::new(id as u32, &formatting.id),
            status: ViewCell::new(status, &formatting.status),
            customer: ViewCell::new(customer, &formatting.customer),
            balance: ViewCell::new(balance, &formatting.balance),
            created_at: ViewCell::new(created_at, &formatting.created_at),
            updated_at: ViewCell::new(updated_at, &formatting.updated_at),
        }
    }
}
