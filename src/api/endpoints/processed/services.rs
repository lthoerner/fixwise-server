use rust_decimal::Decimal;
use serde::Serialize;

use proc_macros::{FromRecord, FromRelation, ProcessEndpoint, ServeEntityJson, ServeRowJson};

use crate::api::endpoints::ViewCell;
use crate::api::GenericIdParameter;
use crate::database::views::services::{ServicesView, ServicesViewRecord};
use crate::database::Relation;

#[derive(FromRelation, ServeEntityJson, Serialize)]
#[endpoint(relation = ServicesView, raw = false)]
pub struct ServicesApiEndpoint {
    metadata: EndpointMetadata,
    rows: Vec<ServicesApiEndpointRow>,
}

#[derive(ProcessEndpoint, FromRecord, ServeRowJson, Serialize)]
#[endpoint_row(id_param = GenericIdParameter, record = ServicesViewRecord, raw = false)]
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
