use serde::Serialize;

use proc_macros::{
    FromDatabaseEntity, FromDatabaseRow, ProcessEndpoint, ServeEntityJson, ServeRowJson,
};

use crate::api::endpoints::ViewCell;
use crate::api::GenericIdParameter;
use crate::database::views::device_models::{
    DeviceModelsDatabaseView, DeviceModelsDatabaseViewRow,
};
use crate::database::DatabaseEntity;

#[derive(FromDatabaseEntity, ServeEntityJson, Serialize)]
#[endpoint(database_entity = DeviceModelsDatabaseView, raw = false)]
pub struct DeviceModelsApiEndpoint {
    metadata: EndpointMetadata,
    rows: Vec<DeviceModelsApiEndpointRow>,
}

#[derive(ProcessEndpoint, FromDatabaseRow, ServeRowJson, Serialize)]
#[endpoint_row(id_param = GenericIdParameter, database_row = DeviceModelsDatabaseViewRow, raw = false)]
pub struct DeviceModelsApiEndpointRow {
    #[col_format(preset = "id")]
    id: ViewCell<i32>,
    #[col_format(preset = "string-notrim", display_name = "Name")]
    display_name: ViewCell<String>,
    #[col_format(preset = "string-notrim")]
    manufacturer: ViewCell<String>,
    #[col_format(preset = "string-notrim")]
    category: ViewCell<String>,
}
