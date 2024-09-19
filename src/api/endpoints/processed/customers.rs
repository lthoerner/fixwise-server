use serde::Serialize;

use proc_macros::{ProcessEndpoint, ServeEntityJson, ServeRowJson};

use crate::api::endpoints::ViewCell;
use crate::api::{FromDatabaseEntity, FromDatabaseRow, GenericIdParameter};
use crate::database::views::customers::{CustomersDatabaseView, CustomersDatabaseViewRow};
use crate::database::DatabaseEntity;

#[derive(ServeEntityJson, Serialize)]
pub struct CustomersApiEndpoint {
    metadata: EndpointMetadata,
    rows: Vec<CustomersApiEndpointRow>,
}

#[derive(ProcessEndpoint, ServeRowJson, Serialize)]
#[id_param(GenericIdParameter)]
pub struct CustomersApiEndpointRow {
    #[col_format(
        format = "id",
        data_type = "integer",
        display_name = "ID",
        trimmable = false
    )]
    id: ViewCell<u32>,
    #[col_format(data_type = "string", trimmable = false)]
    name: ViewCell<String>,
    #[col_format(data_type = "string", trimmable = true)]
    email_address: ViewCell<Option<String>>,
    #[col_format(data_type = "string", trimmable = true)]
    phone_number: ViewCell<Option<String>>,
    #[col_format(data_type = "string", trimmable = true)]
    street_address: ViewCell<Option<String>>,
}

impl FromDatabaseEntity for CustomersApiEndpoint {
    type Entity = CustomersDatabaseView;
    fn from_database_entity(entity: Self::Entity) -> Self {
        Self {
            metadata: EndpointMetadata::new(),
            rows: entity
                .take_rows()
                .into_iter()
                .map(CustomersApiEndpointRow::from_database_row)
                .collect(),
        }
    }
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
