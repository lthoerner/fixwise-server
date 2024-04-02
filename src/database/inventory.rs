use std::collections::HashSet;
use std::fmt::Debug;

use axum::extract::State;
use axum::Json;
use rand::thread_rng;
use rand::Rng;
use rust_decimal::Decimal;
use serde::Serialize;
use sqlx::{query, Row};

use super::api::CellValue;
use crate::ServerState;

#[derive(Debug, Clone, Serialize)]
pub struct InventoryItem {
    pub(super) sku: CellValue<i32>,
    pub(super) display_name: CellValue<String>,
    pub(super) count: CellValue<i32>,
    pub(super) cost: CellValue<Decimal>,
    pub(super) price: CellValue<Decimal>,
}

impl InventoryItem {
    pub fn generate(existing: &mut HashSet<i32>) -> Self {
        let sku = crate::generate_unique_random_i32(0, existing);
        let count = thread_rng().gen_range(1..=999);
        let cost = Decimal::new(thread_rng().gen_range(10000..=99999), 2);
        let price = cost * Decimal::new(thread_rng().gen_range(2..=5), 0);

        Self {
            sku: CellValue::new(sku, None),
            display_name: CellValue::new(Self::generate_display_name(), None),
            count: CellValue::new(count, None),
            cost: CellValue::new(cost, None),
            price: CellValue::new(price, None),
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
}

pub async fn get_inventory(State(state): State<ServerState>) -> Json<Vec<InventoryItem>> {
    let inventory_rows = query("SELECT * FROM test.inventory ORDER BY sku")
        .fetch_all(&state.database.connection)
        .await
        .unwrap();

    let view_configuration = state.view_configurations.inventory.backend;
    let sku_formatting = view_configuration.get_column_formatting("sku");
    let display_name_formatting = view_configuration.get_column_formatting("display_name");
    let count_formatting = view_configuration.get_column_formatting("count");
    let cost_formatting = view_configuration.get_column_formatting("cost");
    let price_formatting = view_configuration.get_column_formatting("price");

    let mut inventory_items = Vec::new();
    for item in inventory_rows {
        inventory_items.push(InventoryItem {
            sku: CellValue::new(item.get("sku"), sku_formatting),
            display_name: CellValue::new(item.get("display_name"), display_name_formatting),
            count: CellValue::new(item.get("count"), count_formatting),
            cost: CellValue::new(item.get("cost"), cost_formatting),
            price: CellValue::new(item.get("price"), price_formatting),
        });
    }

    Json(inventory_items)
}
