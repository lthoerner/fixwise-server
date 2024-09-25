use serde::Serialize;

use proc_macros::{FromRecord, FromRelation, ProcessEndpoint, ServeEntityJson, ServeRowJson};

use crate::api::endpoints::ViewCell;
use crate::api::GenericIdParameter;
use crate::database::views::devices::{DevicesView, DevicesViewRecord};
use crate::database::Relation;

#[derive(FromRelation, ServeEntityJson, Serialize)]
#[endpoint(relation = DevicesView, raw = false)]
pub struct DevicesApiEndpoint {
    metadata: EndpointMetadata,
    rows: Vec<DevicesApiEndpointRow>,
}

#[derive(ProcessEndpoint, FromRecord, ServeRowJson, Serialize)]
#[endpoint_row(id_param = GenericIdParameter, record = DevicesViewRecord, raw = false)]
pub struct DevicesApiEndpointRow {
    #[col_format(preset = "id")]
    id: ViewCell<i32>,
    #[col_format(preset = "string-notrim")]
    model: ViewCell<String>,
    #[col_format(preset = "string")]
    owner: ViewCell<Option<String>>,
}
