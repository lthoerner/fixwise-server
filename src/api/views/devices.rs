use serde::Serialize;

use super::{
    ColumnFormat, FrontendColumnDisplay, FrontendColumnMetadata, FrontendDataType, ViewCell,
};
use crate::api::{DatabaseEntity, FromDatabaseEntity};
use crate::database::views::devices::{DevicesDatabaseView, DevicesDatabaseViewRow};

#[derive(Serialize)]
pub struct DevicesApiView {
    metadata: DevicesApiViewMetadata,
    rows: Vec<DevicesApiViewRow>,
}

#[derive(Serialize)]
struct DevicesApiViewRow {
    id: ViewCell<u32>,
    model: ViewCell<String>,
    owner: ViewCell<Option<String>>,
}

struct DevicesApiViewFormatting {
    id: ColumnFormat,
    model: ColumnFormat,
    owner: ColumnFormat,
}

#[derive(Serialize)]
struct DevicesApiViewMetadata {
    id: FrontendColumnMetadata,
    model: FrontendColumnMetadata,
    owner: FrontendColumnMetadata,
}

impl DevicesApiViewFormatting {
    const fn new() -> Self {
        Self {
            id: ColumnFormat::Id,
            model: ColumnFormat::None,
            owner: ColumnFormat::None,
        }
    }
}

impl DevicesApiViewMetadata {
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

impl FromDatabaseEntity for DevicesApiView {
    type Entity = DevicesDatabaseView;
    fn from_database_entity(entity: Self::Entity) -> Self {
        let formatting = DevicesApiViewFormatting::new();
        Self {
            metadata: DevicesApiViewMetadata::new(),
            rows: entity
                .take_rows()
                .into_iter()
                .map(|row| {
                    let DevicesDatabaseViewRow { id, model, owner } = row;
                    DevicesApiViewRow {
                        id: ViewCell::new(id as u32, &formatting.id),
                        model: ViewCell::new(model, &formatting.model),
                        owner: ViewCell::new(owner, &formatting.owner),
                    }
                })
                .collect(),
        }
    }
}
