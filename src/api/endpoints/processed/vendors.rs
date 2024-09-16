use serde::Serialize;

use crate::api::endpoints::{
    ColumnFormat, FrontendColumnDisplay, FrontendColumnMetadata, FrontendDataType, ViewCell,
};
use crate::api::{
    FromDatabaseEntity, FromDatabaseRow, GenericIdParameter, ServeEntityJson, ServeRowJson,
};
use crate::database::views::vendors::{VendorsDatabaseView, VendorsDatabaseViewRow};
use crate::database::DatabaseEntity;

#[derive(Serialize)]
pub struct VendorsApiEndpoint {
    metadata: EndpointMetadata,
    rows: Vec<VendorsApiEndpointRow>,
}

#[derive(Serialize)]
pub struct VendorsApiEndpointRow {
    id: ViewCell<u32>,
    display_name: ViewCell<String>,
}

struct EndpointFormatting {
    id: ColumnFormat,
    display_name: ColumnFormat,
}

#[derive(Serialize)]
struct EndpointMetadata {
    id: FrontendColumnMetadata,
    display_name: FrontendColumnMetadata,
}

impl EndpointFormatting {
    const fn new() -> Self {
        Self {
            id: ColumnFormat::Id,
            display_name: ColumnFormat::None,
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
        }
    }
}

impl ServeEntityJson for VendorsApiEndpoint {}
impl FromDatabaseEntity for VendorsApiEndpoint {
    type Entity = VendorsDatabaseView;
    fn from_database_entity(entity: Self::Entity) -> Self {
        Self {
            metadata: EndpointMetadata::new(),
            rows: entity
                .take_rows()
                .into_iter()
                .map(VendorsApiEndpointRow::from_database_row)
                .collect(),
        }
    }
}

impl ServeRowJson<GenericIdParameter> for VendorsApiEndpointRow {}
impl FromDatabaseRow for VendorsApiEndpointRow {
    type Row = VendorsDatabaseViewRow;
    fn from_database_row(row: Self::Row) -> Self {
        let formatting = EndpointFormatting::new();
        let VendorsDatabaseViewRow { id, display_name } = row;
        VendorsApiEndpointRow {
            id: ViewCell::new(id as u32, &formatting.id),
            display_name: ViewCell::new(display_name, &formatting.display_name),
        }
    }
}
