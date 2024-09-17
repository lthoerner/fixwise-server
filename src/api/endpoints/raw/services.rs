use rust_decimal::Decimal;
use serde::Serialize;

use proc_macros::{ServeEntityJson, ServeRowJson};

use crate::api::{FromDatabaseEntity, FromDatabaseRow, GenericIdParameter};
use crate::database::views::services::{ServicesDatabaseView, ServicesDatabaseViewRow};
use crate::database::DatabaseEntity;

#[derive(ServeEntityJson, Serialize)]
pub struct ServicesApiEndpoint {
    rows: Vec<ServicesApiEndpointRow>,
}

#[derive(ServeRowJson, Serialize)]
#[id_param(GenericIdParameter)]
pub struct ServicesApiEndpointRow {
    pub id: i32,
    pub type_name: String,
    pub device_name: String,
    pub base_fee: Decimal,
    pub labor_fee: Decimal,
}

impl FromDatabaseEntity for ServicesApiEndpoint {
    type Entity = ServicesDatabaseView;
    fn from_database_entity(entity: Self::Entity) -> Self {
        Self {
            rows: entity
                .take_rows()
                .into_iter()
                .map(ServicesApiEndpointRow::from_database_row)
                .collect(),
        }
    }
}

impl FromDatabaseRow for ServicesApiEndpointRow {
    type Row = ServicesDatabaseViewRow;
    fn from_database_row(row: Self::Row) -> Self {
        ServicesApiEndpointRow {
            id: row.id,
            type_name: row.type_name,
            device_name: row.device_name,
            base_fee: row.base_fee,
            labor_fee: row.labor_fee,
        }
    }
}
