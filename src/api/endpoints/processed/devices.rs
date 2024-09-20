use serde::Serialize;

use proc_macros::{FromDatabaseEntity, ProcessEndpoint, ServeEntityJson, ServeRowJson};

use crate::api::endpoints::ViewCell;
use crate::api::{FromDatabaseRow, GenericIdParameter};
use crate::database::views::devices::{DevicesDatabaseView, DevicesDatabaseViewRow};
use crate::database::DatabaseEntity;

#[derive(FromDatabaseEntity, ServeEntityJson, Serialize)]
#[database_entity(DevicesDatabaseView)]
pub struct DevicesApiEndpoint {
    metadata: EndpointMetadata,
    rows: Vec<DevicesApiEndpointRow>,
}

#[derive(ProcessEndpoint, ServeRowJson, Serialize)]
#[id_param(GenericIdParameter)]
pub struct DevicesApiEndpointRow {
    #[col_format(
        format = "id",
        data_type = "integer",
        display_name = "ID",
        trimmable = false
    )]
    id: ViewCell<u32>,
    #[col_format(data_type = "string", trimmable = false)]
    model: ViewCell<String>,
    #[col_format(data_type = "string", trimmable = true)]
    owner: ViewCell<Option<String>>,
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
