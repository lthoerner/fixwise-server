use rust_decimal::Decimal;
use serde::Serialize;

use proc_macros::{FromRecord, FromRelation, ServeRecordJson, ServeResourceJson};

use crate::api::GenericIdParameter;
use crate::database::views::products::{ProductsView, ProductsViewRecord};
use crate::database::Relation;

#[derive(FromRelation, ServeResourceJson, Serialize)]
#[resource(relation = ProductsView, raw = true)]
pub struct ProductsResource {
    records: Vec<ProductsResourceRecord>,
}

#[derive(FromRecord, ServeRecordJson, Serialize)]
#[resource_record(id_param = GenericIdParameter, record = ProductsViewRecord, raw = true)]
pub struct ProductsResourceRecord {
    pub sku: i32,
    pub display_name: String,
    pub cost: Decimal,
    pub price: Decimal,
}
