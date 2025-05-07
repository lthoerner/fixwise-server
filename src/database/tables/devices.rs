use std::collections::HashSet;

use crudkit::prelude::*;
use serde::Serialize;

use proc_macros::GenerateTable;

use super::customers::CustomersTable;
use super::device_models::DeviceModelsTable;
use super::generators::*;
use crate::database::traits::GenerateRecord;

#[derive(Relation, ReadRelation, WriteRelation, BulkInsert, GenerateTable, Serialize, Clone)]
#[relation(relation_name = "devices", primary_key = "id")]
pub struct DevicesTable {
    records: Vec<DevicesTableRecord>,
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
pub struct DevicesTableRecord {
    #[auto_primary_key]
    #[defaultable]
    pub id: Option<i32>,
    pub model: i32,
    pub owner: Option<i32>,
}

impl GenerateRecord for DevicesTableRecord {
    type Identifier = i32;
    type Dependencies<'a> = (&'a DeviceModelsTable, &'a CustomersTable);
    fn generate(
        _existing_records: &[Self],
        existing_ids: &mut HashSet<Self::Identifier>,
        dependencies: Self::Dependencies<'_>,
    ) -> Self {
        Self {
            id: Some(generate_unique_i32(0, existing_ids)),
            model: dependencies.0.pick_random().id().unwrap(),
            owner: generate_option(dependencies.1.pick_random().id().unwrap(), 0.9),
        }
    }
}
