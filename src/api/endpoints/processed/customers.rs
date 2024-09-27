use serde::Serialize;

use proc_macros::{FromRecord, FromRelation, ProcessEndpoint, ServeRecordJson, ServeResourceJson};

use crate::api::endpoints::ViewCell;
use crate::api::GenericIdParameter;
use crate::database::views::customers::{CustomersView, CustomersViewRecord};
use crate::database::Relation;

#[derive(FromRelation, ServeResourceJson, Serialize)]
#[resource(relation = CustomersView, raw = false)]
pub struct CustomersResource {
    metadata: EndpointMetadata,
    records: Vec<CustomersResourceRecord>,
}

#[derive(ProcessEndpoint, FromRecord, ServeRecordJson, Serialize)]
#[resource_record(id_param = GenericIdParameter, record = CustomersViewRecord, raw = false)]
pub struct CustomersResourceRecord {
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
