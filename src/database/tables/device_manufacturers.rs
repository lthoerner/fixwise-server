use std::collections::HashSet;

use proc_macros::{
    BulkInsert, CreateAndUpdate, GenerateTable, IdentifiableRecord, ReadRelation, Relation,
    SingleInsert, WriteRelation,
};

use super::generators::*;
use crate::database::GenerateRecord;

#[derive(Relation, ReadRelation, WriteRelation, BulkInsert, GenerateTable, Clone)]
#[relation(relation_name = "device_manufacturers", primary_key = "id")]
pub struct DeviceManufacturersTable {
    records: Vec<DeviceManufacturersTableRecord>,
}

#[derive(SingleInsert, CreateAndUpdate, sqlx::FromRow, IdentifiableRecord, Clone)]
pub struct DeviceManufacturersTableRecord {
    #[auto_primary_key]
    pub id: i32,
    pub display_name: String,
}

impl GenerateRecord for DeviceManufacturersTableRecord {
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
