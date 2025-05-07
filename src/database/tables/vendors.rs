use std::collections::HashSet;

use crudkit::{
    BulkInsert, IdentifiableRecord, ReadRecord, ReadRelation, Record, Relation, SingleInsert,
    WriteRecord, WriteRelation,
};
use serde::Serialize;

use proc_macros::GenerateTable;

use super::generators::*;
use crate::database::traits::GenerateRecord;

#[derive(Relation, ReadRelation, WriteRelation, BulkInsert, GenerateTable, Serialize, Clone)]
#[relation(relation_name = "vendors", primary_key = "id")]
pub struct VendorsTable {
    records: Vec<VendorsTableRecord>,
}

#[derive(
    Record,
    ReadRecord,
    WriteRecord,
    SingleInsert,
    Serialize,
    sqlx::FromRow,
    IdentifiableRecord,
    Clone,
)]
pub struct VendorsTableRecord {
    #[auto_primary_key]
    #[defaultable]
    pub id: Option<i32>,
    pub display_name: String,
    pub email_address: Option<String>,
    pub phone_number: Option<String>,
    pub street_address: Option<String>,
}

impl GenerateRecord for VendorsTableRecord {
    type Identifier = i32;
    type Dependencies<'a> = ();
    fn generate(
        _existing_records: &[Self],
        existing_ids: &mut HashSet<Self::Identifier>,
        _dependencies: Self::Dependencies<'_>,
    ) -> Self {
        Self {
            id: Some(generate_unique_i32(0, existing_ids)),
            display_name: generate_company_name(),
            email_address: generate_option(generate_email_address(), 0.7),
            phone_number: generate_option(generate_phone_number(), 0.5),
            street_address: generate_option(generate_street_address(), 0.2),
        }
    }
}
