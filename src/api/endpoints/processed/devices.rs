use serde::Serialize;

use proc_macros::{
    FromDatabaseEntity, FromDatabaseRow, ProcessEndpoint, ServeEntityJson, ServeRowJson,
};

use crate::api::endpoints::ViewCell;
use crate::api::GenericIdParameter;
use crate::database::views::devices::{DevicesDatabaseView, DevicesDatabaseViewRow};
use crate::database::DatabaseEntity;

#[derive(FromDatabaseEntity, ServeEntityJson, Serialize)]
#[endpoint(database_entity = DevicesDatabaseView, raw = false)]
pub struct DevicesApiEndpoint {
    metadata: EndpointMetadata,
    rows: Vec<DevicesApiEndpointRow>,
}

#[derive(ProcessEndpoint, FromDatabaseRow, ServeRowJson, Serialize)]
#[endpoint_row(id_param = GenericIdParameter, database_row = DevicesDatabaseViewRow, raw = false)]
pub struct DevicesApiEndpointRow {
    #[col_format(preset = "id")]
    id: ViewCell<i32>,
    #[col_format(preset = "string-notrim")]
    model: ViewCell<String>,
    #[col_format(preset = "string")]
    owner: ViewCell<Option<String>>,
}
