use std::collections::HashSet;

use crudkit::prelude::*;
use serde::Serialize;

use proc_macros::GenerateTable;

use super::generators::*;
use crate::database::traits::GenerateRecord;

#[derive(Relation, ReadRelation, WriteRelation, BulkInsert, GenerateTable, Serialize, Clone)]
#[relation(relation_name = "customers", primary_key = "id")]
pub struct CustomersTable {
    records: Vec<CustomersTableRecord>,
}

#[derive(
    Record,
    ReadRecord,
    WriteRecord,
    SingleInsert,
    Serialize,
    sqlx::FromRow,
    Clone,
    IdentifiableRecord,
)]
pub struct CustomersTableRecord {
    #[auto_primary_key]
    #[defaultable]
    pub id: Option<i32>,
    pub name: String,
    pub email_address: Option<String>,
    pub phone_number: Option<String>,
    pub street_address: Option<String>,
}

impl GenerateRecord for CustomersTableRecord {
    type Identifier = i32;
    type Dependencies<'a> = ();
    fn generate(
        _existing_records: &[Self],
        existing_ids: &mut HashSet<Self::Identifier>,
        _dependencies: Self::Dependencies<'_>,
    ) -> Self {
        Self {
            id: Some(generate_unique_i32(0, existing_ids)),
            name: generate_name(),
            email_address: generate_option(generate_email_address(), 0.9),
            phone_number: generate_option(generate_phone_number(), 0.9),
            street_address: generate_option(generate_street_address(), 0.9),
        }
    }
}
