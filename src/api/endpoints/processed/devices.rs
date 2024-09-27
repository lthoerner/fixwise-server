use serde::Serialize;

use proc_macros::{FromRecord, FromRelation, ProcessEndpoint, ServeRecordJson, ServeResourceJson};

use crate::api::endpoints::ViewCell;
use crate::api::GenericIdParameter;
use crate::database::views::devices::{DevicesView, DevicesViewRecord};
use crate::database::Relation;

#[derive(FromRelation, ServeResourceJson, Serialize)]
#[resource(relation = DevicesView, raw = false)]
pub struct DevicesResource {
    metadata: EndpointMetadata,
    records: Vec<DevicesResourceRecord>,
}

#[derive(ProcessEndpoint, FromRecord, ServeRecordJson, Serialize)]
#[resource_record(id_param = GenericIdParameter, record = DevicesViewRecord, raw = false)]
pub struct DevicesResourceRecord {
    #[col_format(preset = "id")]
    id: ViewCell<i32>,
    #[col_format(preset = "string-notrim")]
    model: ViewCell<String>,
    #[col_format(preset = "string")]
    owner: ViewCell<Option<String>>,
}
