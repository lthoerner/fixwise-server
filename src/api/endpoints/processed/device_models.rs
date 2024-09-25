use serde::Serialize;

use proc_macros::{FromRecord, FromRelation, ProcessEndpoint, ServeEntityJson, ServeRowJson};

use crate::api::endpoints::ViewCell;
use crate::api::GenericIdParameter;
use crate::database::views::device_models::{DeviceModelsView, DeviceModelsViewRecord};
use crate::database::Relation;

#[derive(FromRelation, ServeEntityJson, Serialize)]
#[endpoint(relation = DeviceModelsView, raw = false)]
pub struct DeviceModelsApiEndpoint {
    metadata: EndpointMetadata,
    rows: Vec<DeviceModelsApiEndpointRow>,
}

#[derive(ProcessEndpoint, FromRecord, ServeRowJson, Serialize)]
#[endpoint_row(id_param = GenericIdParameter, record = DeviceModelsViewRecord, raw = false)]
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
