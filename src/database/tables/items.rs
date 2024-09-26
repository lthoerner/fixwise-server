use chrono::NaiveDateTime;
use rust_decimal::Decimal;

use proc_macros::{BulkInsert, IdentifiableRecord, Relation, SingleInsert};

use super::product_prices::ProductPricesTable;
use super::service_prices::ServicePricesTable;
use crate::database::shared_models::ItemType;
use crate::database::Relation;

#[derive(Relation, BulkInsert, Clone)]
#[relation(relation_name = "items", primary_key = "id", foreign_key_name = "item")]
pub struct ItemsTable {
    records: Vec<ItemsTableRecord>,
}

impl ItemsTable {
    pub fn get_item_price_by_id(
        &self,
        item_id: i32,
        product_prices: &ProductPricesTable,
        service_prices: &ServicePricesTable,
        timestamp: NaiveDateTime,
    ) -> Decimal {
        self.records
            .iter()
            .find(|r| r.id == item_id)
            // TODO: Remove this unwrap (probably)
            .unwrap()
            .get_item_price_at_time(product_prices, service_prices, timestamp)
    }
}

#[derive(SingleInsert, sqlx::FromRow, IdentifiableRecord, Clone)]
pub struct ItemsTableRecord {
    pub id: i32,
    pub product_or_service: i32,
    #[sqlx(rename = "type")]
    pub r#type: ItemType,
}

impl ItemsTableRecord {
    pub fn get_item_price_at_time(
        &self,
        product_prices: &ProductPricesTable,
        service_prices: &ServicePricesTable,
        timestamp: NaiveDateTime,
    ) -> Decimal {
        let product_or_service_id = self.product_or_service;
        match self.r#type {
            ItemType::Product => {
                let most_recent_price = product_prices
                    .records()
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
                    .records()
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
