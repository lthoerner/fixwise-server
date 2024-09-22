use serde::Serialize;

use proc_macros::{
    FromDatabaseEntity, FromDatabaseRow, ProcessEndpoint, ServeEntityJson, ServeRowJson,
};

use crate::api::endpoints::ViewCell;
use crate::api::GenericIdParameter;
use crate::database::views::customers::{CustomersDatabaseView, CustomersDatabaseViewRow};
use crate::database::DatabaseEntity;

#[derive(FromDatabaseEntity, ServeEntityJson, Serialize)]
#[database_entity(CustomersDatabaseView)]
pub struct CustomersApiEndpoint {
    metadata: EndpointMetadata,
    rows: Vec<CustomersApiEndpointRow>,
}

#[derive(ProcessEndpoint, FromDatabaseRow, ServeRowJson, Serialize)]
#[endpoint_row(id_param = GenericIdParameter, database_row = CustomersDatabaseViewRow)]
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
