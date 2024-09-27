use serde::Serialize;

use proc_macros::{FromRecord, FromRelation, ProcessEndpoint, ServeRecordJson, ServeResourceJson};

use crate::api::endpoints::ViewCell;
use crate::api::GenericIdParameter;
use crate::database::views::vendors::{VendorsView, VendorsViewRecord};
use crate::database::Relation;

#[derive(FromRelation, ServeResourceJson, Serialize)]
#[resource(relation = VendorsView, raw = false)]
pub struct VendorsResource {
    metadata: EndpointMetadata,
    records: Vec<VendorsResourceRecord>,
}

#[derive(ProcessEndpoint, FromRecord, ServeRecordJson, Serialize)]
#[resource_record(id_param = GenericIdParameter, record = VendorsViewRecord, raw = false)]
pub struct VendorsResourceRecord {
    #[col_format(preset = "id")]
    id: ViewCell<i32>,
    #[col_format(preset = "string-notrim", display_name = "Name")]
    display_name: ViewCell<String>,
}
