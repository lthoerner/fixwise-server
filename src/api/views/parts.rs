use rust_decimal::Decimal;
use serde::Serialize;

use super::{
    ColumnFormat, FrontendColumnDisplay, FrontendColumnMetadata, FrontendDataType, ViewCell,
};
use crate::api::{
    FromDatabaseEntity, FromDatabaseRow, GenericIdParameter, ServeEntityJson, ServeRowJson,
};
use crate::database::views::parts::{PartsDatabaseView, PartsDatabaseViewRow};
use crate::database::DatabaseEntity;

#[derive(Serialize)]
pub struct PartsApiView {
    metadata: PartsApiViewMetadata,
    rows: Vec<PartsApiViewRow>,
}

#[derive(Serialize)]
pub struct PartsApiViewRow {
    id: ViewCell<u32>,
    display_name: ViewCell<String>,
    vendor: ViewCell<String>,
    manufacturer: ViewCell<Option<String>>,
    category: ViewCell<String>,
    cost: ViewCell<Option<Decimal>>,
    price: ViewCell<Option<Decimal>>,
}

struct PartsApiViewFormatting {
    id: ColumnFormat,
    display_name: ColumnFormat,
    vendor: ColumnFormat,
    manufacturer: ColumnFormat,
    category: ColumnFormat,
    cost: ColumnFormat,
    price: ColumnFormat,
}

#[derive(Serialize)]
struct PartsApiViewMetadata {
    id: FrontendColumnMetadata,
    display_name: FrontendColumnMetadata,
    vendor: FrontendColumnMetadata,
    manufacturer: FrontendColumnMetadata,
    category: FrontendColumnMetadata,
    cost: FrontendColumnMetadata,
    price: FrontendColumnMetadata,
}

impl PartsApiViewFormatting {
    const fn new() -> Self {
        Self {
            id: ColumnFormat::Id,
            display_name: ColumnFormat::None,
            vendor: ColumnFormat::None,
            manufacturer: ColumnFormat::None,
            category: ColumnFormat::None,
            cost: ColumnFormat::Currency,
            price: ColumnFormat::Currency,
        }
    }
}

impl PartsApiViewMetadata {
    const fn new() -> Self {
        Self {
            id: FrontendColumnMetadata {
                data_type: FrontendDataType::Integer,
                display: FrontendColumnDisplay::Text {
                    name: "ID",
                    trimmable: false,
                },
            },
            display_name: FrontendColumnMetadata {
                data_type: FrontendDataType::String,
                display: FrontendColumnDisplay::Text {
                    name: "Name",
                    trimmable: false,
                },
            },
            vendor: FrontendColumnMetadata {
                data_type: FrontendDataType::String,
                display: FrontendColumnDisplay::Text {
                    name: "Vendor",
                    trimmable: true,
                },
            },
            manufacturer: FrontendColumnMetadata {
                data_type: FrontendDataType::String,
                display: FrontendColumnDisplay::Text {
                    name: "Manufacturer",
                    trimmable: true,
                },
            },
            category: FrontendColumnMetadata {
                data_type: FrontendDataType::String,
                display: FrontendColumnDisplay::Text {
                    name: "Category",
                    trimmable: true,
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

impl ServeEntityJson for PartsApiView {}
impl FromDatabaseEntity for PartsApiView {
    type Entity = PartsDatabaseView;
    fn from_database_entity(entity: Self::Entity) -> Self {
        Self {
            metadata: PartsApiViewMetadata::new(),
            rows: entity
                .take_rows()
                .into_iter()
                .map(PartsApiViewRow::from_database_row)
                .collect(),
        }
    }
}

impl ServeRowJson<GenericIdParameter> for PartsApiViewRow {}
impl FromDatabaseRow for PartsApiViewRow {
    type Row = PartsDatabaseViewRow;
    fn from_database_row(row: Self::Row) -> Self {
        let formatting = PartsApiViewFormatting::new();

        let PartsDatabaseViewRow {
            id,
            display_name,
            vendor,
            manufacturer,
            category,
            cost,
            price,
        } = row;

        PartsApiViewRow {
            id: ViewCell::new(id as u32, &formatting.id),
            display_name: ViewCell::new(display_name, &formatting.display_name),
            vendor: ViewCell::new(vendor, &formatting.vendor),
            manufacturer: ViewCell::new(manufacturer, &formatting.manufacturer),
            category: ViewCell::new(category, &formatting.category),
            cost: ViewCell::new(cost, &formatting.cost),
            price: ViewCell::new(price, &formatting.price),
        }
    }
}
