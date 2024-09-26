use std::collections::HashSet;

use chrono::NaiveDateTime;
use rust_decimal::prelude::ToPrimitive;
use rust_decimal::Decimal;

use proc_macros::{BulkInsert, Relation, GenerateTableData, IdentifiableRecord, SingleInsert};

use super::generators::*;
use super::products::ProductsTable;
use super::IdentifiableRecord;
use crate::database::{GenerateRecord, Relation};

#[derive(Relation, BulkInsert, GenerateTableData, Clone)]
#[relation(
    relation_name = "product_prices",
    primary_key = "id",
    foreign_key_name = "product_price"
)]
pub struct ProductPricesTable {
    records: Vec<ProductPricesTableRecord>,
}

#[derive(SingleInsert, sqlx::FromRow, IdentifiableRecord, Clone)]
pub struct ProductPricesTableRecord {
    pub id: i32,
    pub product: i32,
    #[defaultable]
    pub cost: Option<Decimal>,
    #[defaultable]
    pub price: Option<Decimal>,
    #[defaultable]
    pub time_set: Option<NaiveDateTime>,
}

impl GenerateRecord for ProductPricesTableRecord {
    type Identifier = i32;
    type Dependencies<'a> = &'a ProductsTable;
    fn generate(
        _existing_records: &[Self],
        existing_ids: &mut HashSet<Self::Identifier>,
        dependencies: Self::Dependencies<'_>,
    ) -> Self {
        let cost = generate_dollar_value(Some(1.00), Some(500.00));
        let price = generate_dollar_value(Some(cost.to_f32().unwrap()), Some(1000.00));

        Self {
            id: generate_unique_i32(0, existing_ids),
            product: dependencies.pick_random().id(),
            cost: Some(cost),
            price: Some(price),
            time_set: Some(generate_date(None)),
        }
    }
}
