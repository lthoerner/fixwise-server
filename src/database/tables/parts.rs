use std::collections::HashSet;

use rust_decimal::prelude::ToPrimitive;
use rust_decimal::Decimal;
use sqlx::query_builder::Separated;
use sqlx::Postgres;

use super::generators::*;
use super::part_categories::PartCategoriesDatabaseTable;
use super::part_manufacturers::PartManufacturersDatabaseTable;
use super::vendors::VendorsDatabaseTable;
use super::IdentifiableRow;
use crate::database::{BulkInsert, DatabaseEntity};

pub struct PartsDatabaseTable {
    rows: Vec<PartsDatabaseTableRow>,
}

impl DatabaseEntity for PartsDatabaseTable {
    type Row = PartsDatabaseTableRow;
    const ENTITY_NAME: &str = "parts";
    const PRIMARY_COLUMN_NAME: &str = "id";

    fn with_rows(rows: Vec<Self::Row>) -> Self {
        Self { rows }
    }

    fn take_rows(self) -> Vec<Self::Row> {
        self.rows
    }

    fn rows(&self) -> &[Self::Row] {
        &self.rows
    }
}

impl BulkInsert for PartsDatabaseTable {
    const COLUMN_NAMES: &[&str] = &[
        "id",
        "display_name",
        "vendor",
        "manufacturer",
        "category",
        "cost",
        "price",
    ];

    fn push_bindings(mut builder: Separated<Postgres, &str>, row: Self::Row) {
        builder
            .push_bind(row.id)
            .push_bind(row.display_name)
            .push_bind(row.vendor)
            .push_bind(row.manufacturer)
            .push_bind(row.category)
            .push_bind(row.cost)
            .push_bind(row.price);
    }
}

#[derive(sqlx::FromRow, Clone)]
pub struct PartsDatabaseTableRow {
    pub id: i32,
    pub display_name: String,
    pub vendor: i32,
    pub manufacturer: Option<i32>,
    pub category: i32,
    pub cost: Option<Decimal>,
    pub price: Option<Decimal>,
}

impl IdentifiableRow for PartsDatabaseTableRow {
    fn id(&self) -> i32 {
        self.id
    }
}

impl PartsDatabaseTable {
    fn generate(
        count: usize,
        existing_vendors: &VendorsDatabaseTable,
        existing_part_manufacturers: &PartManufacturersDatabaseTable,
        existing_part_categories: &PartCategoriesDatabaseTable,
    ) -> Self {
        let mut rows = Vec::new();
        let mut existing_ids = HashSet::new();
        for _ in 0..count {
            rows.push(PartsDatabaseTableRow::generate(
                &mut existing_ids,
                existing_vendors,
                existing_part_manufacturers,
                existing_part_categories,
            ));
        }

        Self::with_rows(rows)
    }
}

impl PartsDatabaseTableRow {
    fn generate(
        existing_ids: &mut HashSet<i32>,
        existing_vendors: &VendorsDatabaseTable,
        existing_part_manufacturers: &PartManufacturersDatabaseTable,
        existing_part_categories: &PartCategoriesDatabaseTable,
    ) -> Self {
        let cost = generate_option(generate_dollar_value(Some(1.00), Some(500.00)), 0.8);
        let price = match cost {
            Some(cost) => Some(generate_dollar_value(
                Some(cost.to_f32().unwrap()),
                Some(1000.00),
            )),
            None => None,
        };

        Self {
            id: generate_unique_i32(0, existing_ids),
            // TODO: Generate via vendor/manufacturer/category data along with compatibilities
            display_name: "PLACEHOLDER".to_owned(),
            vendor: existing_vendors.pick_random().id(),
            manufacturer: generate_option(existing_part_manufacturers.pick_random().id(), 0.2),
            category: existing_part_categories.pick_random().id(),
            cost,
            price,
        }
    }
}
