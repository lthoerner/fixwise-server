use serde::Serialize;

use proc_macros::{ProcessEndpoint, ServeEntityJson, ServeRowJson};

use crate::api::endpoints::ViewCell;
use crate::api::{FromDatabaseEntity, FromDatabaseRow, GenericIdParameter};
use crate::database::views::device_models::{
    DeviceModelsDatabaseView, DeviceModelsDatabaseViewRow,
};
use crate::database::DatabaseEntity;

#[derive(ServeEntityJson, Serialize)]
pub struct DeviceModelsApiEndpoint {
    metadata: EndpointMetadata,
    rows: Vec<DeviceModelsApiEndpointRow>,
}

#[derive(ProcessEndpoint, ServeRowJson, Serialize)]
#[id_param(GenericIdParameter)]
pub struct DeviceModelsApiEndpointRow {
    #[col_format(
        format = "id",
        data_type = "integer",
        display_name = "ID",
        trimmable = false
    )]
    id: ViewCell<u32>,
    #[col_format(data_type = "string", display_name = "Name", trimmable = false)]
    display_name: ViewCell<String>,
    #[col_format(data_type = "string", trimmable = false)]
    manufacturer: ViewCell<String>,
    #[col_format(data_type = "string", trimmable = false)]
    category: ViewCell<String>,
}

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
