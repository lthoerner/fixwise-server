use serde::Serialize;

use crate::api::endpoints::{
    ColumnFormat, FrontendColumnDisplay, FrontendColumnMetadata, FrontendDataType, ViewCell,
};
use crate::api::{
    FromDatabaseEntity, FromDatabaseRow, GenericIdParameter, ServeEntityJson, ServeRowJson,
};
use crate::database::views::device_models::{
    DeviceModelsDatabaseView, DeviceModelsDatabaseViewRow,
};
use crate::database::DatabaseEntity;

#[derive(Serialize)]
pub struct DeviceModelsApiEndpoint {
    metadata: EndpointMetadata,
    rows: Vec<DeviceModelsApiEndpointRow>,
}

#[derive(Serialize)]
pub struct DeviceModelsApiEndpointRow {
    id: ViewCell<u32>,
    display_name: ViewCell<String>,
    manufacturer: ViewCell<String>,
    category: ViewCell<String>,
}

struct EndpointFormatting {
    id: ColumnFormat,
    display_name: ColumnFormat,
    manufacturer: ColumnFormat,
    category: ColumnFormat,
}

#[derive(Serialize)]
struct EndpointMetadata {
    id: FrontendColumnMetadata,
    display_name: FrontendColumnMetadata,
    manufacturer: FrontendColumnMetadata,
    category: FrontendColumnMetadata,
}

impl EndpointFormatting {
    const fn new() -> Self {
        Self {
            id: ColumnFormat::Id,
            display_name: ColumnFormat::None,
            manufacturer: ColumnFormat::None,
            category: ColumnFormat::None,
        }
    }
}

impl EndpointMetadata {
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
            manufacturer: FrontendColumnMetadata {
                data_type: FrontendDataType::String,
                display: FrontendColumnDisplay::Text {
                    name: "Manufacturer",
                    trimmable: false,
                },
            },
            category: FrontendColumnMetadata {
                data_type: FrontendDataType::String,
                display: FrontendColumnDisplay::Text {
                    name: "Category",
                    trimmable: false,
                },
            },
        }
    }
}

impl ServeEntityJson for DeviceModelsApiEndpoint {}
impl FromDatabaseEntity for DeviceModelsApiEndpoint {
    type Entity = DeviceModelsDatabaseView;
    fn from_database_entity(entity: Self::Entity) -> Self {
        Self {
            metadata: EndpointMetadata::new(),
            rows: entity
                .take_rows()
                .into_iter()
                .map(DeviceModelsApiEndpointRow::from_database_row)
                .collect(),
        }
    }
}

impl ServeRowJson<GenericIdParameter> for DeviceModelsApiEndpointRow {}
impl FromDatabaseRow for DeviceModelsApiEndpointRow {
    type Row = DeviceModelsDatabaseViewRow;
    fn from_database_row(row: Self::Row) -> Self {
        let formatting = EndpointFormatting::new();

        let DeviceModelsDatabaseViewRow {
            id,
            display_name,
            manufacturer,
            category,
        } = row;

        DeviceModelsApiEndpointRow {
            id: ViewCell::new(id as u32, &formatting.id),
            display_name: ViewCell::new(display_name, &formatting.display_name),
            manufacturer: ViewCell::new(manufacturer, &formatting.manufacturer),
            category: ViewCell::new(category, &formatting.category),
        }
    }
}
