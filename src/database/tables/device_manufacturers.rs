use std::collections::HashSet;

use crudkit::prelude::*;
use serde::Serialize;

use proc_macros::GenerateTable;

use super::generators::*;
use crate::database::traits::GenerateRecord;

#[derive(Relation, ReadRelation, WriteRelation, BulkInsert, GenerateTable, Serialize, Clone)]
#[relation(relation_name = "device_manufacturers", primary_key = "id")]
pub struct DeviceManufacturersTable {
    records: Vec<DeviceManufacturersTableRecord>,
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
pub struct DeviceManufacturersTableRecord {
    #[auto_primary_key]
    #[defaultable]
    pub id: Option<i32>,
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
            id: Some(generate_unique_i32(0, existing_ids)),
            display_name: generate_company_name(),
        }
    }
}
