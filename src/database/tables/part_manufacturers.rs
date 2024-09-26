use std::collections::HashSet;

use proc_macros::{
    BulkInsert, GenerateTableData, IdentifiableRecord, Relation, SingleInsert, Table,
};

use super::generators::*;
use crate::database::GenerateRecord;

#[derive(Relation, Table, BulkInsert, GenerateTableData, Clone)]
#[relation(relation_name = "part_manufacturers", primary_key = "id")]
pub struct PartManufacturersTable {
    records: Vec<PartManufacturersTableRecord>,
}

#[derive(SingleInsert, sqlx::FromRow, IdentifiableRecord, Clone)]
pub struct PartManufacturersTableRecord {
    pub id: i32,
    pub display_name: String,
}

impl GenerateRecord for PartManufacturersTableRecord {
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
        }
    }
}
