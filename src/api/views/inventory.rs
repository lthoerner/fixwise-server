use rust_decimal::Decimal;
use serde::Serialize;

use super::{ColumnFormat, ViewCell};
use crate::api::{DatabaseEntity, FromDatabaseEntity};
use crate::database::views::inventory::{InventoryDatabaseView, InventoryDatabaseViewRow};

#[derive(Serialize)]
pub struct InventoryApiView {
    rows: Vec<InventoryApiViewRow>,
}

#[derive(Serialize)]
struct InventoryApiViewRow {
    sku: ViewCell<u32>,
    name: ViewCell<String>,
    count: ViewCell<u32>,
    price: ViewCell<Decimal>,
    cost: ViewCell<Decimal>,
}

struct InventoryFormatting {
    sku: ColumnFormat,
    name: ColumnFormat,
    count: ColumnFormat,
    price: ColumnFormat,
    cost: ColumnFormat,
}

impl InventoryFormatting {
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

impl FromDatabaseEntity for InventoryApiView {
    type Entity = InventoryDatabaseView;
    fn from_database_entity(entity: Self::Entity) -> Self {
        let formatting = InventoryFormatting::new();
        Self {
            rows: entity
                .rows()
                .into_iter()
                .map(|row| {
                    let InventoryDatabaseViewRow {
                        sku,
                        name,
                        count,
                        price,
                        cost,
                    } = row;

                    InventoryApiViewRow {
                        sku: ViewCell::new(sku as u32, &formatting.sku),
                        name: ViewCell::new(name, &formatting.name),
                        count: ViewCell::new(count as u32, &formatting.count),
                        price: ViewCell::new(price, &formatting.price),
                        cost: ViewCell::new(cost, &formatting.cost),
                    }
                })
                .collect(),
        }
    }
}
