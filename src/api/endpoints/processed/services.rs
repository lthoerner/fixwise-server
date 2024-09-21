use rust_decimal::Decimal;
use serde::Serialize;

use proc_macros::{FromDatabaseEntity, ProcessEndpoint, ServeEntityJson, ServeRowJson};

use crate::api::endpoints::ViewCell;
use crate::api::{FromDatabaseRow, GenericIdParameter};
use crate::database::views::services::{ServicesDatabaseView, ServicesDatabaseViewRow};
use crate::database::DatabaseEntity;

#[derive(FromDatabaseEntity, ServeEntityJson, Serialize)]
#[database_entity(ServicesDatabaseView)]
pub struct ServicesApiEndpoint {
    metadata: EndpointMetadata,
    rows: Vec<ServicesApiEndpointRow>,
}

#[derive(ProcessEndpoint, ServeRowJson, Serialize)]
#[id_param(GenericIdParameter)]
pub struct ServicesApiEndpointRow {
    #[col_format(preset = "id")]
    id: ViewCell<u32>,
    #[col_format(preset = "string-notrim", display_name = "Type")]
    type_name: ViewCell<String>,
    #[col_format(preset = "string-notrim", display_name = "Device")]
    device_name: ViewCell<String>,
    #[col_format(preset = "currency")]
    base_fee: ViewCell<Decimal>,
    #[col_format(preset = "currency")]
    labor_fee: ViewCell<Decimal>,
}

impl FromDatabaseRow for ServicesApiEndpointRow {
    type Row = ServicesDatabaseViewRow;
    fn from_database_row(row: Self::Row) -> Self {
        let formatting = EndpointFormatting::new();

        let ServicesDatabaseViewRow {
            id,
            type_name,
            device_name,
            base_fee,
            labor_fee,
        } = row;

        ServicesApiEndpointRow {
            id: ViewCell::new(id as u32, &formatting.id),
            type_name: ViewCell::new(type_name, &formatting.type_name),
            device_name: ViewCell::new(device_name, &formatting.device_name),
            base_fee: ViewCell::new(base_fee, &formatting.base_fee),
            labor_fee: ViewCell::new(labor_fee, &formatting.labor_fee),
        }
    }
}
