use rust_decimal::Decimal;
use serde::Serialize;

use proc_macros::{FromDatabaseEntity, FromDatabaseRow, ServeEntityJson, ServeRowJson};

use crate::api::GenericIdParameter;
use crate::database::views::products::{ProductsDatabaseView, ProductsDatabaseViewRow};
use crate::database::DatabaseEntity;

#[derive(FromDatabaseEntity, ServeEntityJson, Serialize)]
#[endpoint(database_entity = ProductsDatabaseView, raw = true)]
pub struct ProductsApiEndpoint {
    rows: Vec<ProductsApiEndpointRow>,
}

#[derive(FromDatabaseRow, ServeRowJson, Serialize)]
#[endpoint_row(id_param = GenericIdParameter, database_row = ProductsDatabaseViewRow, raw = true)]
pub struct ProductsApiEndpointRow {
    pub sku: i32,
    pub display_name: String,
    pub cost: Decimal,
    pub price: Decimal,
}
