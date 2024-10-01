use rust_decimal::Decimal;
use serde::Serialize;

use proc_macros::Relation;

use crate::database::shared_models::ItemType;

#[derive(Relation, Serialize)]
#[relation(relation_name = "items_view", primary_key = "item_id")]
pub struct ItemsView {
    records: Vec<ItemsViewRecord>,
}

#[derive(sqlx::FromRow, Serialize, Clone)]
pub struct ItemsViewRecord {
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
