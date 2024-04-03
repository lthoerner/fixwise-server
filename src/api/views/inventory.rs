use rust_decimal::Decimal;
use serde::Serialize;

use super::{ColumnFormat, ViewCell};
use crate::api::FromDatabaseRow;
use crate::database::views::inventory::InventoryItem as DatabaseInventoryItem;

#[derive(Serialize)]
pub struct InventoryItem {
    sku: ViewCell<u32>,
    name: ViewCell<String>,
    count: ViewCell<u32>,
    price: ViewCell<Decimal>,
    cost: ViewCell<Decimal>,
}

struct InventoryItemFormatting {
    sku: ColumnFormat,
    name: ColumnFormat,
    count: ColumnFormat,
    price: ColumnFormat,
    cost: ColumnFormat,
}

impl InventoryItemFormatting {
    const fn new() -> Self {
        Self {
            sku: ColumnFormat::Id,
            name: ColumnFormat::None,
            count: ColumnFormat::None,
            price: ColumnFormat::Currency,
            cost: ColumnFormat::Currency,
        }
    }
}

impl FromDatabaseRow for InventoryItem {
    type Entity = DatabaseInventoryItem;
    fn from_database_row(row: Self::Entity) -> Self {
        let formatting = InventoryItemFormatting::new();
        let Self::Entity {
            sku,
            name,
            count,
            price,
            cost,
        } = row;

        Self {
            sku: ViewCell::new(sku as u32, formatting.sku),
            name: ViewCell::new(name, formatting.name),
            count: ViewCell::new(count as u32, formatting.count),
            price: ViewCell::new(price, formatting.price),
            cost: ViewCell::new(cost, formatting.cost),
        }
    }
}
