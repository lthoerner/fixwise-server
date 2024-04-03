use serde::Serialize;

use super::{
    ColumnFormat, FrontendColumnDisplay, FrontendColumnMetadata, FrontendDataType, ViewCell,
};
use crate::api::{DatabaseEntity, FromDatabaseEntity};
use crate::database::views::customers::{CustomersDatabaseView, CustomersDatabaseViewRow};

#[derive(Serialize)]
pub struct CustomersApiView {
    metadata: CustomersApiViewMetadata,
    rows: Vec<CustomersApiViewRow>,
}

#[derive(Serialize)]
struct CustomersApiViewRow {
    id: ViewCell<u32>,
    name: ViewCell<String>,
    email: ViewCell<String>,
    phone: ViewCell<String>,
    address: ViewCell<Option<String>>,
}

struct CustomersApiViewFormatting {
    id: ColumnFormat,
    name: ColumnFormat,
    email: ColumnFormat,
    phone: ColumnFormat,
    address: ColumnFormat,
}

#[derive(Serialize)]
struct CustomersApiViewMetadata {
    id: FrontendColumnMetadata,
    name: FrontendColumnMetadata,
    email: FrontendColumnMetadata,
    phone: FrontendColumnMetadata,
    address: FrontendColumnMetadata,
}

impl CustomersApiViewFormatting {
    const fn new() -> Self {
        Self {
            id: ColumnFormat::Id,
            name: ColumnFormat::None,
            email: ColumnFormat::None,
            phone: ColumnFormat::None,
            address: ColumnFormat::None,
        }
    }
}

impl CustomersApiViewMetadata {
    const fn new() -> Self {
        Self {
            id: FrontendColumnMetadata {
                data_type: FrontendDataType::Integer,
                display: FrontendColumnDisplay {
                    name: "ID",
                    trimmable: false,
                },
            },
            name: FrontendColumnMetadata {
                data_type: FrontendDataType::String,
                display: FrontendColumnDisplay {
                    name: "Name",
                    trimmable: false,
                },
            },
            email: FrontendColumnMetadata {
                data_type: FrontendDataType::String,
                display: FrontendColumnDisplay {
                    name: "Email Address",
                    trimmable: true,
                },
            },
            phone: FrontendColumnMetadata {
                data_type: FrontendDataType::String,
                display: FrontendColumnDisplay {
                    name: "Phone Number",
                    trimmable: true,
                },
            },
            address: FrontendColumnMetadata {
                data_type: FrontendDataType::String,
                display: FrontendColumnDisplay {
                    name: "Mailing Address",
                    trimmable: true,
                },
            },
        }
    }
}

impl FromDatabaseEntity for CustomersApiView {
    type Entity = CustomersDatabaseView;
    fn from_database_entity(entity: Self::Entity) -> Self {
        let formatting = CustomersApiViewFormatting::new();
        Self {
            metadata: CustomersApiViewMetadata::new(),
            rows: entity
                .rows()
                .into_iter()
                .map(|row| {
                    let CustomersDatabaseViewRow {
                        id,
                        name,
                        email,
                        phone,
                        address,
                    } = row;

                    CustomersApiViewRow {
                        id: ViewCell::new(id as u32, &formatting.id),
                        name: ViewCell::new(name, &formatting.name),
                        email: ViewCell::new(email, &formatting.email),
                        phone: ViewCell::new(phone, &formatting.phone),
                        address: ViewCell::new(address, &formatting.address),
                    }
                })
                .collect(),
        }
    }
}
