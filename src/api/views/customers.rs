use serde::Serialize;

use super::{ColumnFormat, ViewCell};
use crate::api::FromDatabaseRow;
use crate::database::views::customers::CustomersDatabaseViewRow;

#[derive(Serialize)]
pub struct CustomersApiViewRow {
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

impl FromDatabaseRow for CustomersApiViewRow {
    type Entity = CustomersDatabaseViewRow;
    fn from_database_row(row: Self::Entity) -> Self {
        let formatting = CustomersFormatting::new();
        let Self::Entity {
            id,
            name,
            email,
            phone,
            address,
        } = row;

        Self {
            id: ViewCell::new(id as u32, formatting.id),
            name: ViewCell::new(name, formatting.name),
            email: ViewCell::new(email, formatting.email),
            phone: ViewCell::new(phone, formatting.phone),
            address: ViewCell::new(address, formatting.address),
        }
    }
}
