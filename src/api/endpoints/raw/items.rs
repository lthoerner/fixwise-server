use rust_decimal::Decimal;
use serde::Serialize;

use proc_macros::{ServeEntityJson, ServeRowJson};

use crate::api::{FromDatabaseEntity, FromDatabaseRow, GenericIdParameter};
use crate::database::shared_models::ItemType;
use crate::database::views::items::{ItemsDatabaseView, ItemsDatabaseViewRow};
use crate::database::DatabaseEntity;

#[derive(ServeEntityJson, Serialize)]
pub struct ItemsApiEndpoint {
    rows: Vec<ItemsApiEndpointRow>,
}

#[derive(ServeRowJson, Serialize)]
#[id_param(GenericIdParameter)]
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

impl FromDatabaseEntity for ItemsApiEndpoint {
    type Entity = ItemsDatabaseView;
    fn from_database_entity(entity: Self::Entity) -> Self {
        Self {
            rows: entity
                .take_rows()
                .into_iter()
                .map(ItemsApiEndpointRow::from_database_row)
                .collect(),
        }
    }
}

impl FromDatabaseRow for ItemsApiEndpointRow {
    type Row = ItemsDatabaseViewRow;
    fn from_database_row(row: Self::Row) -> Self {
        ItemsApiEndpointRow {
            item_id: row.item_id,
            item_type: row.item_type,
            product_sku: row.product_sku,
            product_name: row.product_name,
            product_cost: row.product_cost,
            product_price: row.product_price,
            service_id: row.service_id,
            service_type_name: row.service_type_name,
            service_device_name: row.service_device_name,
            service_base_fee: row.service_base_fee,
            service_labor_fee: row.service_labor_fee,
        }
    }
}
