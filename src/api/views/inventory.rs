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
    price: ViewCell<Decimal>,
    cost: ViewCell<Decimal>,
}

struct InventoryApiViewFormatting {
    sku: ColumnFormat,
    name: ColumnFormat,
    count: ColumnFormat,
    price: ColumnFormat,
    cost: ColumnFormat,
}

#[derive(Serialize)]
struct InventoryApiViewMetadata {
    sku: FrontendColumnMetadata,
    name: FrontendColumnMetadata,
    count: FrontendColumnMetadata,
    price: FrontendColumnMetadata,
    cost: FrontendColumnMetadata,
}

impl InventoryApiViewFormatting {
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

impl InventoryApiViewMetadata {
    const fn new() -> Self {
        Self {
            sku: FrontendColumnMetadata {
                data_type: FrontendDataType::Integer,
                display: FrontendColumnDisplay {
                    name: "SKU",
                    trimmable: false,
                },
            },
            name: FrontendColumnMetadata {
                data_type: FrontendDataType::String,
                display: FrontendColumnDisplay {
                    name: "Name",
                    trimmable: false,
                },
            },
            count: FrontendColumnMetadata {
                data_type: FrontendDataType::Integer,
                display: FrontendColumnDisplay {
                    name: "Count",
                    trimmable: false,
                },
            },
            cost: FrontendColumnMetadata {
                data_type: FrontendDataType::Decimal,
                display: FrontendColumnDisplay {
                    name: "Cost",
                    trimmable: false,
                },
            },
            price: FrontendColumnMetadata {
                data_type: FrontendDataType::Decimal,
                display: FrontendColumnDisplay {
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
