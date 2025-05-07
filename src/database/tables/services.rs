use std::collections::HashSet;

use crudkit::prelude::*;
use serde::Serialize;

use proc_macros::GenerateTable;

use super::device_models::DeviceModelsTable;
use super::generators::*;
use super::service_types::ServiceTypesTable;
use crate::database::traits::GenerateRecord;

#[derive(Relation, ReadRelation, WriteRelation, BulkInsert, GenerateTable, Serialize, Clone)]
#[relation(relation_name = "services", primary_key = "id")]
pub struct ServicesTable {
    records: Vec<ServicesTableRecord>,
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
pub struct ServicesTableRecord {
    #[auto_primary_key]
    #[defaultable]
    pub id: Option<i32>,
    #[sqlx(rename = "type")]
    pub r#type: i32,
    pub device: i32,
}

impl GenerateRecord for ServicesTableRecord {
    type Identifier = i32;
    type Dependencies<'a> = (&'a ServiceTypesTable, &'a DeviceModelsTable);
    fn generate(
        _existing_records: &[Self],
        existing_ids: &mut HashSet<Self::Identifier>,
        dependencies: Self::Dependencies<'_>,
    ) -> Self {
        Self {
            id: Some(generate_unique_i32(0, existing_ids)),
            r#type: dependencies.0.pick_random().id().unwrap(),
            device: dependencies.1.pick_random().id().unwrap(),
        }
    }
}
