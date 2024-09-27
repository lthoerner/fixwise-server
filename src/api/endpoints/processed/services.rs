use rust_decimal::Decimal;
use serde::Serialize;

use proc_macros::{FromRecord, FromRelation, ProcessEndpoint, ServeRecordJson, ServeResourceJson};

use crate::api::endpoints::ViewCell;
use crate::api::GenericIdParameter;
use crate::database::views::services::{ServicesView, ServicesViewRecord};
use crate::database::Relation;

#[derive(FromRelation, ServeResourceJson, Serialize)]
#[resource(relation = ServicesView, raw = false)]
pub struct ServicesResource {
    metadata: EndpointMetadata,
    records: Vec<ServicesResourceRecord>,
}

#[derive(ProcessEndpoint, FromRecord, ServeRecordJson, Serialize)]
#[resource_record(id_param = GenericIdParameter, record = ServicesViewRecord, raw = false)]
pub struct ServicesResourceRecord {
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
