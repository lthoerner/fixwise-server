use rust_decimal::Decimal;
use serde::Serialize;

use super::{
    ColumnFormat, FrontendColumnDisplay, FrontendColumnMetadata, FrontendDataType, ViewCell,
};
use crate::api::{DatabaseEntity, FromDatabaseEntity};
use crate::database::views::inventory::{InventoryDatabaseView, InventoryDatabaseViewRow};

#[derive(Serialize)]
pub struct InventoryApiView {
    metadata: InventoryApiViewMetadata,
    rows: Vec<InventoryApiViewRow>,
}

#[derive(Serialize)]
struct InventoryApiViewRow {
    sku: ViewCell<u32>,
    name: ViewCell<String>,
    count: ViewCell<u32>,
    cost: ViewCell<Decimal>,
    price: ViewCell<Decimal>,
}

struct InventoryApiViewFormatting {
    sku: ColumnFormat,
    name: ColumnFormat,
    count: ColumnFormat,
    cost: ColumnFormat,
    price: ColumnFormat,
}

#[derive(Serialize)]
struct InventoryApiViewMetadata {
    sku: FrontendColumnMetadata,
    name: FrontendColumnMetadata,
    count: FrontendColumnMetadata,
    cost: FrontendColumnMetadata,
    price: FrontendColumnMetadata,
}

impl InventoryApiViewFormatting {
    const fn new() -> Self {
        Self {
            sku: ColumnFormat::Id,
            name: ColumnFormat::None,
            count: ColumnFormat::None,
            cost: ColumnFormat::Currency,
            price: ColumnFormat::Currency,
        }
    }
}

impl InventoryApiViewMetadata {
    const fn new() -> Self {
        Self {
            sku: FrontendColumnMetadata {
                data_type: FrontendDataType::Integer,
                display: FrontendColumnDisplay::Text {
                    name: "SKU",
                    trimmable: false,
                },
            },
            name: FrontendColumnMetadata {
                data_type: FrontendDataType::String,
                display: FrontendColumnDisplay::Text {
                    name: "Name",
                    trimmable: false,
                },
            },
            count: FrontendColumnMetadata {
                data_type: FrontendDataType::Integer,
                display: FrontendColumnDisplay::Text {
                    name: "Count",
                    trimmable: false,
                },
            },
            cost: FrontendColumnMetadata {
                data_type: FrontendDataType::Decimal,
                display: FrontendColumnDisplay::Text {
                    name: "Cost",
                    trimmable: false,
                },
            },
            price: FrontendColumnMetadata {
                data_type: FrontendDataType::Decimal,
                display: FrontendColumnDisplay::Text {
                    name: "Price",
                    trimmable: false,
                },
            },
        }
    }
}

impl FromDatabaseEntity for InventoryApiView {
    type Entity = InventoryDatabaseView;
    fn from_database_entity(entity: Self::Entity) -> Self {
        let formatting = InventoryApiViewFormatting::new();
        Self {
            metadata: InventoryApiViewMetadata::new(),
            rows: entity
                .rows()
                .into_iter()
                .map(|row| {
                    let InventoryDatabaseViewRow {
                        sku,
                        name,
                        count,
                        cost,
                        price,
                    } = row;

                    InventoryApiViewRow {
                        sku: ViewCell::new(sku as u32, &formatting.sku),
                        name: ViewCell::new(name, &formatting.name),
                        count: ViewCell::new(count as u32, &formatting.count),
                        cost: ViewCell::new(cost, &formatting.cost),
                        price: ViewCell::new(price, &formatting.price),
                    }
                })
                .collect(),
        }
    }
}
