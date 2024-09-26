use rust_decimal::Decimal;
use serde::Serialize;

use proc_macros::{FromRecord, FromRelation, ServeResourceJson, ServeRecordJson};

use crate::api::GenericIdParameter;
use crate::database::views::services::{ServicesView, ServicesViewRecord};
use crate::database::Relation;

#[derive(FromRelation, ServeResourceJson, Serialize)]
#[resource(relation = ServicesView, raw = true)]
pub struct ServicesResource {
    records: Vec<ServicesResourceRecord>,
}

#[derive(FromRecord, ServeRecordJson, Serialize)]
#[resource_record(id_param = GenericIdParameter, record = ServicesViewRecord, raw = true)]
pub struct ServicesResourceRecord {
    pub id: i32,
    pub type_name: String,
    pub device_name: String,
    pub base_fee: Decimal,
    pub labor_fee: Decimal,
}
