use std::collections::HashSet;

use rust_decimal::prelude::ToPrimitive;
use rust_decimal::Decimal;

use proc_macros::{
    BulkInsert, GenerateTable, IdentifiableRecord, Relation, SingleInsert, Table,
};

use super::generators::*;
use super::part_categories::PartCategoriesTable;
use super::part_manufacturers::PartManufacturersTable;
use super::vendors::VendorsTable;
use super::IdentifiableRecord;
use crate::database::{GenerateRecord, Relation};

#[derive(Relation, Table, BulkInsert, GenerateTable, Clone)]
#[relation(relation_name = "parts", primary_key = "id")]
pub struct PartsTable {
    records: Vec<PartsTableRecord>,
}

#[derive(SingleInsert, sqlx::FromRow, IdentifiableRecord, Clone)]
pub struct PartsTableRecord {
    pub id: i32,
    pub display_name: String,
    pub vendor: i32,
    pub manufacturer: Option<i32>,
    pub category: i32,
    #[defaultable]
    pub cost: Option<Decimal>,
    #[defaultable]
    pub price: Option<Decimal>,
}

impl GenerateRecord for PartsTableRecord {
    type Identifier = i32;
    type Dependencies<'a> = (
        &'a VendorsTable,
        &'a PartManufacturersTable,
        &'a PartCategoriesTable,
    );

    fn generate(
        _existing_records: &[Self],
        existing_ids: &mut HashSet<Self::Identifier>,
        dependencies: Self::Dependencies<'_>,
    ) -> Self {
        let cost = generate_dollar_value(Some(1.00), Some(500.00));
        let price = generate_dollar_value(Some(cost.to_f32().unwrap()), Some(1000.00));

        Self {
            id: generate_unique_i32(0, existing_ids),
            // TODO: Generate via vendor/manufacturer/category data along with compatibilities
            display_name: "PLACEHOLDER".to_owned(),
            vendor: dependencies.0.pick_random().id(),
            manufacturer: generate_option(dependencies.1.pick_random().id(), 0.2),
            category: dependencies.2.pick_random().id(),
            cost: Some(cost),
            price: Some(price),
        }
    }
}
