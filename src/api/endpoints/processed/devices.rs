use serde::Serialize;

use proc_macros::{ServeEntityJson, ServeRowJson};

use crate::api::endpoints::{
    ColumnFormat, FrontendColumnDisplay, FrontendColumnMetadata, FrontendDataType, ViewCell,
};
use crate::api::{FromDatabaseEntity, FromDatabaseRow, GenericIdParameter};
use crate::database::views::devices::{DevicesDatabaseView, DevicesDatabaseViewRow};
use crate::database::DatabaseEntity;

#[derive(ServeEntityJson, Serialize)]
pub struct DevicesApiEndpoint {
    metadata: EndpointMetadata,
    rows: Vec<DevicesApiEndpointRow>,
}

#[derive(ServeRowJson, Serialize)]
#[id_param(GenericIdParameter)]
pub struct DevicesApiEndpointRow {
    id: ViewCell<u32>,
    model: ViewCell<String>,
    owner: ViewCell<Option<String>>,
}

struct EndpointFormatting {
    id: ColumnFormat,
    model: ColumnFormat,
    owner: ColumnFormat,
}

#[derive(Serialize)]
struct EndpointMetadata {
    id: FrontendColumnMetadata,
    model: FrontendColumnMetadata,
    owner: FrontendColumnMetadata,
}

impl EndpointFormatting {
    const fn new() -> Self {
        Self {
            id: ColumnFormat::Id,
            model: ColumnFormat::None,
            owner: ColumnFormat::None,
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
            model: FrontendColumnMetadata {
                data_type: FrontendDataType::String,
                display: FrontendColumnDisplay::Text {
                    name: "Model",
                    trimmable: false,
                },
            },
            owner: FrontendColumnMetadata {
                data_type: FrontendDataType::String,
                display: FrontendColumnDisplay::Text {
                    name: "Owner",
                    trimmable: true,
                },
            },
        }
    }
}

impl FromDatabaseEntity for DevicesApiEndpoint {
    type Entity = DevicesDatabaseView;
    fn from_database_entity(entity: Self::Entity) -> Self {
        Self {
            metadata: EndpointMetadata::new(),
            rows: entity
                .take_rows()
                .into_iter()
                .map(DevicesApiEndpointRow::from_database_row)
                .collect(),
        }
    }
}

impl FromDatabaseRow for DevicesApiEndpointRow {
    type Row = DevicesDatabaseViewRow;
    fn from_database_row(row: Self::Row) -> Self {
        let formatting = EndpointFormatting::new();
        let DevicesDatabaseViewRow { id, model, owner } = row;
        DevicesApiEndpointRow {
            id: ViewCell::new(id as u32, &formatting.id),
            model: ViewCell::new(model, &formatting.model),
            owner: ViewCell::new(owner, &formatting.owner),
        }
    }
}
