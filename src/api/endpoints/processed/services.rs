use rust_decimal::Decimal;
use serde::Serialize;

use proc_macros::{
    FromDatabaseEntity, FromDatabaseRow, ProcessEndpoint, ServeEntityJson, ServeRowJson,
};

use crate::api::endpoints::ViewCell;
use crate::api::GenericIdParameter;
use crate::database::views::services::{ServicesDatabaseView, ServicesDatabaseViewRow};
use crate::database::DatabaseEntity;

#[derive(FromDatabaseEntity, ServeEntityJson, Serialize)]
#[database_entity(ServicesDatabaseView)]
pub struct ServicesApiEndpoint {
    metadata: EndpointMetadata,
    rows: Vec<ServicesApiEndpointRow>,
}

#[derive(ProcessEndpoint, FromDatabaseRow, ServeRowJson, Serialize)]
#[endpoint_row(id_param = GenericIdParameter, database_row = ServicesDatabaseViewRow)]
pub struct ServicesApiEndpointRow {
    #[col_format(preset = "id")]
    id: ViewCell<i32>,
    #[col_format(preset = "string-notrim", display_name = "Type")]
    type_name: ViewCell<String>,
    #[col_format(preset = "string-notrim", display_name = "Device")]
    device_name: ViewCell<String>,
    #[col_format(preset = "currency")]
    base_fee: ViewCell<Decimal>,
    #[col_format(preset = "currency")]
    labor_fee: ViewCell<Decimal>,
}
