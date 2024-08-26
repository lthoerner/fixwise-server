use serde::Serialize;

use super::{
    ColumnFormat, FrontendColumnDisplay, FrontendColumnMetadata, FrontendDataType, ViewCell,
};
use crate::api::{
    FromDatabaseEntity, FromDatabaseRow, GenericIdParameter, ServeEntityJson, ServeRowJson,
};
use crate::database::views::vendors::{VendorsDatabaseView, VendorsDatabaseViewRow};
use crate::database::DatabaseEntity;

#[derive(Serialize)]
pub struct VendorsApiView {
    metadata: VendorsApiViewMetadata,
    rows: Vec<VendorsApiViewRow>,
}

#[derive(Serialize)]
pub struct VendorsApiViewRow {
    id: ViewCell<u32>,
    display_name: ViewCell<String>,
}

struct VendorsApiViewFormatting {
    id: ColumnFormat,
    display_name: ColumnFormat,
}

#[derive(Serialize)]
struct VendorsApiViewMetadata {
    id: FrontendColumnMetadata,
    display_name: FrontendColumnMetadata,
}

impl VendorsApiViewFormatting {
    const fn new() -> Self {
        Self {
            id: ColumnFormat::Id,
            display_name: ColumnFormat::None,
        }
    }
}

impl VendorsApiViewMetadata {
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

impl ServeEntityJson for VendorsApiView {}
impl FromDatabaseEntity for VendorsApiView {
    type Entity = VendorsDatabaseView;
    fn from_database_entity(entity: Self::Entity) -> Self {
        Self {
            metadata: VendorsApiViewMetadata::new(),
            rows: entity
                .take_rows()
                .into_iter()
                .map(VendorsApiViewRow::from_database_row)
                .collect(),
        }
    }
}

impl ServeRowJson<GenericIdParameter> for VendorsApiViewRow {}
impl FromDatabaseRow for VendorsApiViewRow {
    type Row = VendorsDatabaseViewRow;
    type Entity = VendorsDatabaseView;
    fn from_database_row(row: Self::Row) -> Self {
        let formatting = VendorsApiViewFormatting::new();
        let VendorsDatabaseViewRow { id, display_name } = row;
        VendorsApiViewRow {
            id: ViewCell::new(id as u32, &formatting.id),
            display_name: ViewCell::new(display_name, &formatting.display_name),
        }
    }
}
