use rust_decimal::Decimal;
use serde::Serialize;

use proc_macros::{FromRecord, FromRelation, ServeRecordJson, ServeResourceJson};

use crate::api::GenericIdParameter;
use crate::database::shared_models::ItemType;
use crate::database::views::items::{ItemsView, ItemsViewRecord};
use crate::database::Relation;

#[derive(FromRelation, ServeResourceJson, Serialize)]
#[resource(relation = ItemsView, raw = true)]
pub struct ItemsResource {
    records: Vec<ItemsResourceRecord>,
}

#[derive(FromRecord, ServeRecordJson, Serialize)]
#[resource_record(id_param = GenericIdParameter, record = ItemsViewRecord, raw = true)]
pub struct ItemsResourceRecord {
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
