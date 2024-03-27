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
    pub fn generate() -> Self {
        let address: StreetAddress = Faker.fake();
        let customer: Customer = Faker.fake();

        Customer {
            id: crate::generate_random_i32(1000000000),
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

    pub fn build_query(&self) -> String {
        format!(
            "INSERT INTO customers (id, name, email, phone, address) VALUES ({}, $1, $2, $3, $4)",
            self.id
        )
    }
}
