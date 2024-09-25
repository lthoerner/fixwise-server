use serde::Serialize;

use proc_macros::{FromRecord, FromRelation, ProcessEndpoint, ServeEntityJson, ServeRowJson};

use crate::api::endpoints::ViewCell;
use crate::api::GenericIdParameter;
use crate::database::views::customers::{CustomersView, CustomersViewRecord};
use crate::database::Relation;

#[derive(FromRelation, ServeEntityJson, Serialize)]
#[endpoint(relation = CustomersView, raw = false)]
pub struct CustomersApiEndpoint {
    metadata: EndpointMetadata,
    rows: Vec<CustomersApiEndpointRow>,
}

#[derive(ProcessEndpoint, FromRecord, ServeRowJson, Serialize)]
#[endpoint_row(id_param = GenericIdParameter, record = CustomersViewRecord, raw = false)]
pub struct CustomersApiEndpointRow {
    #[col_format(preset = "id")]
    id: ViewCell<i32>,
    #[col_format(preset = "string-notrim")]
    name: ViewCell<String>,
    #[col_format(preset = "string")]
    email_address: ViewCell<Option<String>>,
    #[col_format(preset = "string")]
    phone_number: ViewCell<Option<String>>,
    #[col_format(preset = "string")]
    street_address: ViewCell<Option<String>>,
}
