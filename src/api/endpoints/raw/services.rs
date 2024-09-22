use rust_decimal::Decimal;
use serde::Serialize;

use proc_macros::{FromDatabaseEntity, FromDatabaseRow, ServeEntityJson, ServeRowJson};

use crate::api::GenericIdParameter;
use crate::database::views::services::{ServicesDatabaseView, ServicesDatabaseViewRow};
use crate::database::DatabaseEntity;

#[derive(FromDatabaseEntity, ServeEntityJson, Serialize)]
#[endpoint(database_entity = ServicesDatabaseView, raw = true)]
pub struct ServicesApiEndpoint {
    rows: Vec<ServicesApiEndpointRow>,
}

#[derive(FromDatabaseRow, ServeRowJson, Serialize)]
#[endpoint_row(id_param = GenericIdParameter, database_row = ServicesDatabaseViewRow, raw = true)]
pub struct ServicesApiEndpointRow {
    pub id: i32,
    pub type_name: String,
    pub device_name: String,
    pub base_fee: Decimal,
    pub labor_fee: Decimal,
}
