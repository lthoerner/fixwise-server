use serde::Serialize;

use crate::api::endpoints::{
    ColumnFormat, FrontendColumnDisplay, FrontendColumnMetadata, FrontendDataType, ViewCell,
};
use crate::api::{
    FromDatabaseEntity, FromDatabaseRow, GenericIdParameter, ServeEntityJson, ServeRowJson,
};
use crate::database::views::customers::{CustomersDatabaseView, CustomersDatabaseViewRow};
use crate::database::DatabaseEntity;

#[derive(Serialize)]
pub struct CustomersApiEndpoint {
    metadata: EndpointMetadata,
    rows: Vec<CustomersApiEndpointRow>,
}

#[derive(Serialize)]
pub struct CustomersApiEndpointRow {
    id: ViewCell<u32>,
    name: ViewCell<String>,
    email_address: ViewCell<Option<String>>,
    phone_number: ViewCell<Option<String>>,
    street_address: ViewCell<Option<String>>,
}

struct EndpointFormatting {
    id: ColumnFormat,
    name: ColumnFormat,
    email_address: ColumnFormat,
    phone_number: ColumnFormat,
    street_address: ColumnFormat,
}

#[derive(Serialize)]
struct EndpointMetadata {
    id: FrontendColumnMetadata,
    name: FrontendColumnMetadata,
    email_address: FrontendColumnMetadata,
    phone_number: FrontendColumnMetadata,
    street_address: FrontendColumnMetadata,
}

impl EndpointFormatting {
    const fn new() -> Self {
        Self {
            id: ColumnFormat::Id,
            name: ColumnFormat::None,
            email_address: ColumnFormat::None,
            phone_number: ColumnFormat::None,
            street_address: ColumnFormat::None,
        }
    }
}

impl EndpointMetadata {
    const fn new() -> Self {
        Self {
            id: FrontendColumnMetadata {
                data_type: FrontendDataType::Integer,
                display: FrontendColumnDisplay::Text {
                    name: "ID",
                    trimmable: false,
                },
            },
            name: FrontendColumnMetadata {
                data_type: FrontendDataType::String,
                display: FrontendColumnDisplay::Text {
                    name: "Name",
                    trimmable: false,
                },
            },
            email_address: FrontendColumnMetadata {
                data_type: FrontendDataType::String,
                display: FrontendColumnDisplay::Text {
                    name: "Email Address",
                    trimmable: true,
                },
            },
            phone_number: FrontendColumnMetadata {
                data_type: FrontendDataType::String,
                display: FrontendColumnDisplay::Text {
                    name: "Phone Number",
                    trimmable: true,
                },
            },
            street_address: FrontendColumnMetadata {
                data_type: FrontendDataType::String,
                display: FrontendColumnDisplay::Text {
                    name: "Street Address",
                    trimmable: true,
                },
            },
        }
    }
}

impl ServeEntityJson for CustomersApiEndpoint {}
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

impl ServeRowJson<GenericIdParameter> for CustomersApiEndpointRow {}
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
