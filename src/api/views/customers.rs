use serde::Serialize;

use super::{
    ColumnFormat, FrontendColumnDisplay, FrontendColumnMetadata, FrontendDataType, ViewCell,
};
use crate::api::{
    FromDatabaseEntity, FromDatabaseRow, GenericIdParameter, ServeEntityJson, ServeRowJson,
};
use crate::database::views::customers::{CustomersDatabaseView, CustomersDatabaseViewRow};
use crate::database::DatabaseEntity;

#[derive(Serialize)]
pub struct CustomersApiView {
    metadata: CustomersApiViewMetadata,
    rows: Vec<CustomersApiViewRow>,
}

#[derive(Serialize)]
pub struct CustomersApiViewRow {
    id: ViewCell<u32>,
    name: ViewCell<String>,
    email_address: ViewCell<Option<String>>,
    phone_number: ViewCell<Option<String>>,
    street_address: ViewCell<Option<String>>,
}

struct CustomersApiViewFormatting {
    id: ColumnFormat,
    name: ColumnFormat,
    email_address: ColumnFormat,
    phone_number: ColumnFormat,
    street_address: ColumnFormat,
}

#[derive(Serialize)]
struct CustomersApiViewMetadata {
    id: FrontendColumnMetadata,
    name: FrontendColumnMetadata,
    email_address: FrontendColumnMetadata,
    phone_number: FrontendColumnMetadata,
    street_address: FrontendColumnMetadata,
}

impl CustomersApiViewFormatting {
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

impl CustomersApiViewMetadata {
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

impl ServeEntityJson for CustomersApiView {}
impl FromDatabaseEntity for CustomersApiView {
    type Entity = CustomersDatabaseView;
    fn from_database_entity(entity: Self::Entity) -> Self {
        Self {
            metadata: CustomersApiViewMetadata::new(),
            rows: entity
                .take_rows()
                .into_iter()
                .map(CustomersApiViewRow::from_database_row)
                .collect(),
        }
    }
}

impl ServeRowJson<GenericIdParameter> for CustomersApiViewRow {}
impl FromDatabaseRow for CustomersApiViewRow {
    type Row = CustomersDatabaseViewRow;
    type Entity = CustomersDatabaseView;
    fn from_database_row(row: Self::Row) -> Self {
        let formatting = CustomersApiViewFormatting::new();

        let CustomersDatabaseViewRow {
            id,
            name,
            email_address,
            phone_number,
            street_address,
        } = row;

        CustomersApiViewRow {
            id: ViewCell::new(id as u32, &formatting.id),
            name: ViewCell::new(name, &formatting.name),
            email_address: ViewCell::new(email_address, &formatting.email_address),
            phone_number: ViewCell::new(phone_number, &formatting.phone_number),
            street_address: ViewCell::new(street_address, &formatting.street_address),
        }
    }
}
