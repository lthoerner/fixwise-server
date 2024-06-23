use std::collections::HashSet;

use rust_decimal::prelude::ToPrimitive;
use rust_decimal::Decimal;

use super::part_categories::PartCategoriesDatabaseTable;
use super::part_manufacturers::PartManufacturersDatabaseTable;
use super::vendors::VendorsDatabaseTable;
use super::IdentifiableRow;
use crate::database::DatabaseEntity;

pub struct PartsDatabaseTable {
    rows: Vec<PartsDatabaseTableRow>,
}

impl DatabaseEntity for PartsDatabaseTable {
    type Row = PartsDatabaseTableRow;
    const ENTITY_NAME: &'static str = "parts";
    const PRIMARY_COLUMN_NAME: &'static str = "id";

    fn with_rows(rows: Vec<Self::Row>) -> Self {
        Self { rows }
    }

    fn rows(self) -> Vec<Self::Row> {
        self.rows
    }

    fn borrow_rows(&self) -> &[Self::Row] {
        &self.rows
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
        let cost =
            super::generate_option(super::generate_dollar_value(Some(1.00), Some(500.00)), 0.8);
        let price = match cost {
            Some(cost) => Some(super::generate_dollar_value(
                Some(cost.to_f32().unwrap()),
                Some(1000.00),
            )),
            None => None,
        };

        Self {
            id: super::generate_unique_i32(0, existing_ids),
            // TODO: Generate via vendor/manufacturer/category data along with compatibilities
            display_name: "PLACEHOLDER".to_owned(),
            vendor: existing_vendors.pick_random().id(),
            manufacturer: super::generate_option(
                existing_part_manufacturers.pick_random().id(),
                0.2,
            ),
            category: existing_part_categories.pick_random().id(),
            cost,
            price,
        }
    }
}
