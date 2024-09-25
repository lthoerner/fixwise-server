use serde::Serialize;

use proc_macros::{FromRecord, FromRelation, ProcessEndpoint, ServeEntityJson, ServeRowJson};

use crate::api::endpoints::ViewCell;
use crate::api::GenericIdParameter;
use crate::database::views::vendors::{VendorsView, VendorsViewRecord};
use crate::database::Relation;

#[derive(FromRelation, ServeEntityJson, Serialize)]
#[endpoint(relation = VendorsView, raw = false)]
pub struct VendorsApiEndpoint {
    metadata: EndpointMetadata,
    rows: Vec<VendorsApiEndpointRow>,
}

#[derive(ProcessEndpoint, FromRecord, ServeRowJson, Serialize)]
#[endpoint_row(id_param = GenericIdParameter, record = VendorsViewRecord, raw = false)]
pub struct VendorsApiEndpointRow {
    #[col_format(preset = "id")]
    id: ViewCell<i32>,
    #[col_format(preset = "string-notrim", display_name = "Name")]
    display_name: ViewCell<String>,
}
