use rust_decimal::Decimal;
use serde::Serialize;

use proc_macros::{FromRecord, FromRelation, ServeEntityJson, ServeRowJson};

use crate::api::GenericIdParameter;
use crate::database::views::services::{ServicesView, ServicesViewRecord};
use crate::database::Relation;

#[derive(FromRelation, ServeEntityJson, Serialize)]
#[endpoint(relation = ServicesView, raw = true)]
pub struct ServicesApiEndpoint {
    rows: Vec<ServicesApiEndpointRow>,
}

#[derive(FromRecord, ServeRowJson, Serialize)]
#[endpoint_row(id_param = GenericIdParameter, record = ServicesViewRecord, raw = true)]
pub struct ServicesApiEndpointRow {
    pub id: i32,
    pub type_name: String,
    pub device_name: String,
    pub base_fee: Decimal,
    pub labor_fee: Decimal,
}
