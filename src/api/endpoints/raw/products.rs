use rust_decimal::Decimal;
use serde::Serialize;

use proc_macros::{FromRecord, FromRelation, ServeEntityJson, ServeRowJson};

use crate::api::GenericIdParameter;
use crate::database::views::products::{ProductsView, ProductsViewRecord};
use crate::database::Relation;

#[derive(FromRelation, ServeEntityJson, Serialize)]
#[endpoint(relation = ProductsView, raw = true)]
pub struct ProductsApiEndpoint {
    rows: Vec<ProductsApiEndpointRow>,
}

#[derive(FromRecord, ServeRowJson, Serialize)]
#[endpoint_row(id_param = GenericIdParameter, record = ProductsViewRecord, raw = true)]
pub struct ProductsApiEndpointRow {
    pub sku: i32,
    pub display_name: String,
    pub cost: Decimal,
    pub price: Decimal,
}
