use axum::Json;
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
    pub fn generate(existing: &[Self]) -> Self {
        let mut sku: i64 = 0;
        let mut first_roll = true;
        while first_roll || existing.iter().any(|e| e.sku == sku) {
            sku = crate::generate_random_i32(0) as i64;
            first_roll = false;
        }

        let count: i64 = thread_rng().gen_range(1..=999);
        let cost = Decimal::new(thread_rng().gen_range(10000..=99999), 2);
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
            "INSERT INTO inventory (sku, display_name, count, cost, price) VALUES ({}, $1, {}, {}, {})",
            self.sku, self.count, self.cost, self.price
        )
    }
}

pub async fn get_inventory() -> Json<Vec<InventoryItem>> {
    let inventory_rows = crate::get_db!()
        .query("SELECT * FROM inventory ORDER BY sku", &[])
        .await
        .unwrap();

    let mut inventory_items = Vec::new();
    for item in inventory_rows {
        inventory_items.push(InventoryItem {
            sku: item.get::<_, i32>("sku") as i64,
            display_name: item.get::<_, String>("display_name"),
            count: item.get::<_, i32>("count") as i64,
            cost: item.get::<_, Decimal>("cost"),
            price: item.get::<_, Decimal>("price"),
        });
    }

    Json(inventory_items)
}
