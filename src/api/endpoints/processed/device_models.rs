use serde::Serialize;

use proc_macros::{FromRecord, FromRelation, ProcessEndpoint, ServeResourceJson, ServeRecordJson};

use crate::api::endpoints::ViewCell;
use crate::api::GenericIdParameter;
use crate::database::views::device_models::{DeviceModelsView, DeviceModelsViewRecord};
use crate::database::Relation;

#[derive(FromRelation, ServeResourceJson, Serialize)]
#[resource(relation = DeviceModelsView, raw = false)]
pub struct DeviceModelsResource {
    metadata: EndpointMetadata,
    records: Vec<DeviceModelsResourceRecord>,
}

#[derive(ProcessEndpoint, FromRecord, ServeRecordJson, Serialize)]
#[resource_record(id_param = GenericIdParameter, record = DeviceModelsViewRecord, raw = false)]
pub struct DeviceModelsResourceRecord {
    #[col_format(preset = "id")]
    id: ViewCell<i32>,
    #[col_format(preset = "string-notrim", display_name = "Name")]
    display_name: ViewCell<String>,
    #[col_format(preset = "string-notrim")]
    manufacturer: ViewCell<String>,
    #[col_format(preset = "string-notrim")]
    category: ViewCell<String>,
}
