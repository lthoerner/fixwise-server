use chrono::NaiveDateTime;
use rust_decimal::Decimal;
use serde::Serialize;

use proc_macros::{FromRecord, FromRelation, ProcessEndpoint, ServeEntityJson, ServeRowJson};

use crate::api::endpoints::{CssColor, TagOption, ViewCell};
use crate::api::GenericIdParameter;
use crate::database::shared_models::TicketStatus;
use crate::database::views::tickets::{TicketsView, TicketsViewRecord};
use crate::database::Relation;

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

#[derive(FromRelation, ServeEntityJson, Serialize)]
#[endpoint(relation = TicketsView, raw = false)]
pub struct TicketsApiEndpoint {
    metadata: EndpointMetadata,
    rows: Vec<TicketsApiEndpointRow>,
}

#[derive(ProcessEndpoint, FromRecord, ServeRowJson, Serialize)]
#[endpoint_row(id_param = GenericIdParameter, record = TicketsViewRecord, raw = false)]
pub struct TicketsApiEndpointRow {
    #[col_format(preset = "id")]
    id: ViewCell<i32>,
    #[col_format(format = "tag", data_type = "tag", tag_options = STATUS_TAG_OPTIONS)]
    status: ViewCell<TicketStatus>,
    #[col_format(preset = "string")]
    customer: ViewCell<Option<String>>,
    #[col_format(preset = "currency")]
    balance: ViewCell<Decimal>,
    #[col_format(preset = "date", display_name = "Created")]
    created_at: ViewCell<NaiveDateTime>,
    #[col_format(preset = "date", display_name = "Updated")]
    updated_at: ViewCell<NaiveDateTime>,
}
