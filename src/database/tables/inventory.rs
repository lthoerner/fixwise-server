use std::collections::{HashMap, HashSet};

use rand::thread_rng;
use rand::Rng;
use rust_decimal::Decimal;
use sqlx::FromRow;

use super::{Generate, IdentifiableRow};
use crate::database::DatabaseEntity;

#[derive(FromRow)]
pub struct InventoryDatabaseTableRow {
    pub sku: i32,
    pub name: String,
    pub count: i32,
    pub cost: Decimal,
    pub price: Decimal,
}

impl DatabaseEntity for InventoryDatabaseTableRow {
    const ENTITY_NAME: &'static str = "inventory";
    const PRIMARY_COLUMN_NAME: &'static str = "sku";
}

impl IdentifiableRow for InventoryDatabaseTableRow {
    fn id(&self) -> i32 {
        self.sku
    }
}

impl Generate for InventoryDatabaseTableRow {
    fn generate<'a>(
        existing: &mut HashSet<i32>,
        _dependencies: &'a HashMap<&'static str, &'a [impl IdentifiableRow]>,
    ) -> Self {
        let cost = Decimal::new(thread_rng().gen_range(10000..=99999), 2);
        let price = cost * Decimal::new(thread_rng().gen_range(2..=5), 0);

        Self {
            sku: crate::generate_unique_random_i32(0, existing),
            name: Self::generate_name(),
            count: thread_rng().gen_range(1..=999),
            cost,
            price,
        }
    }
}

impl InventoryDatabaseTableRow {
    fn generate_name() -> String {
        const PHONE_LINES: [&str; 8] = [
            "iPhone",
            "Samsung Galaxy",
            "Google Pixel",
            "Motorola G",
            "LG",
            "Nokia",
            "Sony Xperia",
            "OnePlus",
        ];

        const MODIFIERS: [&str; 8] = ["Pro", "Max", "Ultra", "Plus", "Lite", "Mini", "X", "Z"];

        let phone = PHONE_LINES[thread_rng().gen_range(0..PHONE_LINES.len())];
        let generation = thread_rng().gen_range(1..=50);
        let modifier = MODIFIERS[thread_rng().gen_range(0..MODIFIERS.len())];

        format!("{} {} {}", phone, generation, modifier)
    }
}
