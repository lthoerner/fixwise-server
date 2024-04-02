use std::collections::HashSet;
use std::fmt::Debug;

use axum::extract::State;
use axum::Json;
use fake::faker::address::en::{CityName, StateAbbr, StreetName, StreetSuffix};
use fake::faker::internet::en::FreeEmail;
use fake::faker::name::en::Name;
use fake::faker::phone_number::en::PhoneNumber;
use fake::{Dummy, Fake, Faker};
use serde::Serialize;
use sqlx::{query, Row};

use super::api::CellValue;
use crate::ServerState;

#[derive(Debug, Clone, Serialize)]
pub struct Customer {
    pub(super) id: CellValue<i32>,
    pub(super) name: CellValue<String>,
    pub(super) email: CellValue<String>,
    pub(super) phone: CellValue<String>,
    pub(super) address: CellValue<String>,
}

#[derive(Dummy)]
struct StreetAddress {
    #[dummy(faker = "1..10000")]
    number: u32,
    #[dummy(faker = "StreetName()")]
    street: String,
    #[dummy(faker = "StreetSuffix()")]
    street_suffix: String,
    #[dummy(faker = "CityName()")]
    city: String,
    #[dummy(faker = "StateAbbr()")]
    state: String,
    #[dummy(faker = "10000..99999")]
    zip: u32,
}

impl Customer {
    pub fn generate(existing: &mut HashSet<i32>) -> Self {
        let id = crate::generate_unique_random_i32(0, existing);
        let name = Name().fake();
        let email = FreeEmail().fake();
        let phone = PhoneNumber().fake();
        let address: StreetAddress = Faker.fake();

        Self {
            id: CellValue::new(id, None),
            name: CellValue::new(name, None),
            email: CellValue::new(email, None),
            phone: CellValue::new(phone, None),
            address: CellValue::new(
                format!(
                    "{} {} {}, {}, {} {}",
                    address.number,
                    address.street,
                    address.street_suffix,
                    address.city,
                    address.state,
                    address.zip
                ),
                None,
            ),
        }
    }
}

pub async fn get_customers(State(state): State<ServerState>) -> Json<Vec<Customer>> {
    let customer_rows = query("SELECT * FROM test.customers_view ORDER BY id")
        .fetch_all(&state.database.connection)
        .await
        .unwrap();

    let view_configuration = state.view_configurations.customers.backend;
    let id_formatting = view_configuration.get_column_formatting("id");
    let name_formatting = view_configuration.get_column_formatting("name");
    let email_formatting = view_configuration.get_column_formatting("email");
    let phone_formatting = view_configuration.get_column_formatting("phone");
    let address_formatting = view_configuration.get_column_formatting("address");

    let mut customers = Vec::new();
    for customer in customer_rows {
        customers.push(Customer {
            id: CellValue::new(customer.get("id"), id_formatting),
            name: CellValue::new(customer.get("name"), name_formatting),
            email: CellValue::new(customer.get("email"), email_formatting),
            phone: CellValue::new(customer.get("phone"), phone_formatting),
            address: CellValue::new(customer.get("address"), address_formatting),
        });
    }

    Json(customers)
}
