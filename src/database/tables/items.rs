use chrono::NaiveDateTime;
use rust_decimal::Decimal;

use proc_macros::{BulkInsert, DatabaseEntity, IdentifiableRow, SingleInsert};

use super::product_prices::ProductPricesDatabaseTable;
use super::service_prices::ServicePricesDatabaseTable;
use crate::database::shared_models::ItemType;
use crate::database::DatabaseEntity;

#[derive(DatabaseEntity, BulkInsert, Clone)]
#[entity(entity_name = "items", primary_key = "id")]
pub struct ItemsDatabaseTable {
    rows: Vec<ItemsDatabaseTableRow>,
}

impl ItemsDatabaseTable {
    pub fn get_item_price_by_id(
        &self,
        item_id: i32,
        product_prices: &ProductPricesDatabaseTable,
        service_prices: &ServicePricesDatabaseTable,
        timestamp: NaiveDateTime,
    ) -> Decimal {
        self.rows
            .iter()
            .find(|r| r.id == item_id)
            // TODO: Remove this unwrap (probably)
            .unwrap()
            .get_item_price_at_time(product_prices, service_prices, timestamp)
    }
}

#[derive(SingleInsert, sqlx::FromRow, IdentifiableRow, Clone)]
pub struct ItemsDatabaseTableRow {
    pub id: i32,
    pub product_or_service: i32,
    #[sqlx(rename = "type")]
    pub r#type: ItemType,
}

impl ItemsDatabaseTableRow {
    pub fn get_item_price_at_time(
        &self,
        product_prices: &ProductPricesDatabaseTable,
        service_prices: &ServicePricesDatabaseTable,
        timestamp: NaiveDateTime,
    ) -> Decimal {
        let product_or_service_id = self.product_or_service;
        match self.r#type {
            ItemType::Product => {
                let most_recent_price = product_prices
                    .rows()
                    .iter()
                    .filter(|p| {
                        p.product == product_or_service_id && p.time_set.unwrap() <= timestamp
                    })
                    .max_by_key(|p| p.time_set);

                match most_recent_price {
                    Some(p) => p.price.unwrap(),
                    None => Decimal::new(0, 2),
                }
            }
            ItemType::Service => {
                let most_recent_price = service_prices
                    .rows()
                    .iter()
                    .filter(|p| {
                        p.service == product_or_service_id && p.time_set.unwrap() <= timestamp
                    })
                    .max_by_key(|p| p.time_set);

                match most_recent_price {
                    Some(p) => p.base_fee.unwrap() + p.labor_fee.unwrap(),
                    None => Decimal::new(0, 2),
                }
            }
        }
    }
}
