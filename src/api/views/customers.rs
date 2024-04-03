use serde::Serialize;

use super::{ColumnFormat, ViewCell};
use crate::api::{DatabaseEntity, FromDatabaseEntity};
use crate::database::views::customers::{CustomersDatabaseView, CustomersDatabaseViewRow};

#[derive(Serialize)]
pub struct CustomersApiView {
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

struct CustomersFormatting {
    id: ColumnFormat,
    name: ColumnFormat,
    email: ColumnFormat,
    phone: ColumnFormat,
    address: ColumnFormat,
}

impl CustomersFormatting {
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

impl FromDatabaseEntity for CustomersApiView {
    type Entity = CustomersDatabaseView;
    fn from_database_entity(entity: Self::Entity) -> Self {
        let formatting = CustomersFormatting::new();
        Self {
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
