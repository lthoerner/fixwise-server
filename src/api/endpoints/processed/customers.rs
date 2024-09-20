use serde::Serialize;

use proc_macros::{FromDatabaseEntity, ProcessEndpoint, ServeEntityJson, ServeRowJson};

use crate::api::endpoints::ViewCell;
use crate::api::{FromDatabaseRow, GenericIdParameter};
use crate::database::views::customers::{CustomersDatabaseView, CustomersDatabaseViewRow};
use crate::database::DatabaseEntity;

#[derive(FromDatabaseEntity, ServeEntityJson, Serialize)]
#[database_entity(CustomersDatabaseView)]
pub struct CustomersApiEndpoint {
    metadata: EndpointMetadata,
    rows: Vec<CustomersApiEndpointRow>,
}

#[derive(ProcessEndpoint, ServeRowJson, Serialize)]
#[id_param(GenericIdParameter)]
pub struct CustomersApiEndpointRow {
    #[col_format(preset = "id")]
    id: ViewCell<u32>,
    #[col_format(preset = "string-notrim")]
    name: ViewCell<String>,
    #[col_format(preset = "string")]
    email_address: ViewCell<Option<String>>,
    #[col_format(preset = "string")]
    phone_number: ViewCell<Option<String>>,
    #[col_format(preset = "string")]
    street_address: ViewCell<Option<String>>,
}

impl FromDatabaseRow for CustomersApiEndpointRow {
    type Row = CustomersDatabaseViewRow;
    fn from_database_row(row: Self::Row) -> Self {
        let formatting = EndpointFormatting::new();

        let CustomersDatabaseViewRow {
            id,
            name,
            email_address,
            phone_number,
            street_address,
        } = row;

        CustomersApiEndpointRow {
            id: ViewCell::new(id as u32, &formatting.id),
            name: ViewCell::new(name, &formatting.name),
            email_address: ViewCell::new(email_address, &formatting.email_address),
            phone_number: ViewCell::new(phone_number, &formatting.phone_number),
            street_address: ViewCell::new(street_address, &formatting.street_address),
        }
    }
}
