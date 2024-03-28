use std::collections::HashSet;

use axum::Json;
use fake::faker::address::en::{CityName, StateAbbr, StreetName, StreetSuffix};
use fake::faker::internet::en::FreeEmail;
use fake::faker::name::en::Name;
use fake::faker::phone_number::en::PhoneNumber;
use fake::{Dummy, Fake, Faker};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, Dummy)]
pub struct Customer {
    pub id: i32,
    #[dummy(faker = "Name()")]
    pub name: String,
    #[dummy(faker = "FreeEmail()")]
    pub email: String,
    #[dummy(faker = "PhoneNumber()")]
    pub phone: String,
    pub address: String,
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
        let address: StreetAddress = Faker.fake();
        let customer: Customer = Faker.fake();

        Customer {
            id,
            address: format!(
                "{} {} {}, {}, {} {}",
                address.number,
                address.street,
                address.street_suffix,
                address.city,
                address.state,
                address.zip
            ),
            ..customer
        }
    }
}

pub async fn get_customers() -> Json<Vec<Customer>> {
    let customer_rows = crate::get_db!()
        .query("SELECT * FROM customers ORDER BY id", &[])
        .await
        .unwrap();

    let mut customers = Vec::new();
    for customer in customer_rows {
        customers.push(Customer {
            id: customer.get::<_, i32>("id"),
            name: customer.get::<_, String>("name"),
            email: customer.get::<_, String>("email"),
            phone: customer.get::<_, String>("phone"),
            address: customer.get::<_, String>("address"),
        });
    }

    Json(customers)
}
