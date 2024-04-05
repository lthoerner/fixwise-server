use std::collections::{HashMap, HashSet};

use fake::faker::address::en::{CityName, StateAbbr, StreetName, StreetSuffix};
use fake::faker::internet::en::FreeEmail;
use fake::faker::name::en::Name;
use fake::faker::phone_number::en::PhoneNumber;
use fake::Fake;
use rand::{thread_rng, Rng};

use super::{Generate, IdentifiableRow};
use crate::database::DatabaseEntity;

struct CustomersDatabaseTable {
    rows: Vec<CustomersDatabaseTableRow>,
}

impl DatabaseEntity for CustomersDatabaseTable {
    type Row = CustomersDatabaseTableRow;
    const ENTITY_NAME: &'static str = "customers";
    const PRIMARY_COLUMN_NAME: &'static str = "id";

    fn with_rows(rows: Vec<Self::Row>) -> Self {
        Self { rows }
    }

    fn rows(self) -> Vec<Self::Row> {
        self.rows
    }
}

#[derive(sqlx::FromRow)]
pub struct CustomersDatabaseTableRow {
    pub id: i32,
    pub name: String,
    pub email: String,
    pub phone: String,
    pub address: Option<String>,
}

impl IdentifiableRow for CustomersDatabaseTableRow {
    fn id(&self) -> i32 {
        self.id
    }
}

impl Generate for CustomersDatabaseTableRow {
    fn generate<'a>(
        existing: &mut HashSet<i32>,
        _dependencies: &'a HashMap<&'static str, &'a [impl IdentifiableRow]>,
    ) -> Self {
        Self {
            id: crate::generate_unique_random_i32(0, existing),
            name: Name().fake(),
            email: FreeEmail().fake(),
            phone: PhoneNumber().fake(),
            address: Some(format!(
                "{} {} {}, {}, {} {}",
                thread_rng().gen_range(1..=9999),
                StreetName().fake::<String>(),
                StreetSuffix().fake::<String>(),
                CityName().fake::<String>(),
                StateAbbr().fake::<String>(),
                thread_rng().gen_range(10000..=99999)
            )),
        }
    }
}
