use std::collections::HashSet;

use proc_macros::{BulkInsert, Relation, GenerateTableData, IdentifiableRecord, SingleInsert};

use super::generators::*;
use crate::database::GenerateRecord;

#[derive(Relation, BulkInsert, GenerateTableData, Clone)]
#[relation(
    relation_name = "vendors",
    primary_key = "id",
    foreign_key_name = "vendor"
)]
pub struct VendorsTable {
    records: Vec<VendorsTableRecord>,
}

#[derive(SingleInsert, sqlx::FromRow, IdentifiableRecord, Clone)]
pub struct VendorsTableRecord {
    pub id: i32,
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
            id: generate_unique_i32(0, existing_ids),
            display_name: generate_company_name(),
            email_address: generate_option(generate_email_address(), 0.7),
            phone_number: generate_option(generate_phone_number(), 0.5),
            street_address: generate_option(generate_street_address(), 0.2),
        }
    }
}
