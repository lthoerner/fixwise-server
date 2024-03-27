use rand::thread_rng;
use rand::Rng;
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InventoryItem {
    pub sku: i64,
    pub display_name: String,
    pub count: i64,
    pub cost: Decimal,
    pub price: Decimal,
}

impl InventoryItem {
    pub fn generate(existing_items: &[Self]) -> Self {
        let mut sku: i64 = 0;
        let mut first_roll = true;
        while first_roll || existing_items.iter().any(|item| item.sku == sku) {
            sku = thread_rng().gen_range(0..=99999999);
            first_roll = false;
        }

        let count: i64 = thread_rng().gen_range(1..=9999);
        let cost = Decimal::new(thread_rng().gen_range(10000..=999999), 2);
        let price = cost * Decimal::new(thread_rng().gen_range(2..=5), 0);

        InventoryItem {
            sku,
            display_name: Self::generate_display_name(),
            count,
            cost,
            price,
        }
    }

    fn generate_display_name() -> String {
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

    pub fn build_query(&self) -> String {
        format!(
            "INSERT INTO inventory (sku, display_name, count, cost, price) VALUES ({}, '{}', {}, {}, {})",
            self.sku, self.display_name, self.count, self.cost, self.price
        )
    }
}
