use rust_decimal::Decimal;
use serde::Serialize;

use proc_macros::{FromRecord, FromRelation, ServeEntityJson, ServeRowJson};

use crate::api::GenericIdParameter;
use crate::database::shared_models::ItemType;
use crate::database::views::items::{ItemsView, ItemsViewRecord};
use crate::database::Relation;

#[derive(FromRelation, ServeEntityJson, Serialize)]
#[endpoint(relation = ItemsView, raw = true)]
pub struct ItemsApiEndpoint {
    rows: Vec<ItemsApiEndpointRow>,
}

#[derive(FromRecord, ServeRowJson, Serialize)]
#[endpoint_row(id_param = GenericIdParameter, record = ItemsViewRecord, raw = true)]
pub struct ItemsApiEndpointRow {
    pub item_id: i32,
    pub item_type: ItemType,
    pub product_sku: Option<i32>,
    pub product_name: Option<String>,
    pub product_cost: Option<Decimal>,
    pub product_price: Option<Decimal>,
    pub service_id: Option<i32>,
    pub service_type_name: Option<String>,
    pub service_device_name: Option<String>,
    pub service_base_fee: Option<Decimal>,
    pub service_labor_fee: Option<Decimal>,
}
