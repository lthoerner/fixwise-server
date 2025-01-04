use std::collections::HashSet;

use proc_macros::{
    BulkInsert, GenerateTable, IdentifiableRecord, ReadRecord, ReadRelation, Record, Relation,
    SingleInsert, WriteRecord, WriteRelation,
};

use super::device_categories::DeviceCategoriesTable;
use super::device_manufacturers::DeviceManufacturersTable;
use super::generators::*;
use super::IdentifiableRecord;
use crate::database::traits::generate::GenerateRecord;
use crate::database::traits::shared::Relation;

#[derive(Relation, ReadRelation, WriteRelation, BulkInsert, GenerateTable, Clone)]
#[relation(relation_name = "device_models", primary_key = "id")]
pub struct DeviceModelsTable {
    records: Vec<DeviceModelsTableRecord>,
}

#[derive(
    Record, ReadRecord, WriteRecord, SingleInsert, sqlx::FromRow, IdentifiableRecord, Clone,
)]
pub struct DeviceModelsTableRecord {
    #[auto_primary_key]
    #[defaultable]
    pub id: Option<i32>,
    pub display_name: String,
    pub primary_model_identifiers: Vec<String>,
    pub secondary_model_identifiers: Vec<String>,
    pub manufacturer: i32,
    pub category: i32,
}

impl GenerateRecord for DeviceModelsTableRecord {
    type Identifier = i32;
    type Dependencies<'a> = (&'a DeviceManufacturersTable, &'a DeviceCategoriesTable);

    fn generate(
        _existing_records: &[Self],
        existing_ids: &mut HashSet<Self::Identifier>,
        dependencies: Self::Dependencies<'_>,
    ) -> Self {
        Self {
            id: Some(generate_unique_i32(0, existing_ids)),
            display_name: generate_device_name(),
            // TODO: Add model identifiers generator
            primary_model_identifiers: Vec::new(),
            secondary_model_identifiers: Vec::new(),
            manufacturer: dependencies.0.pick_random().id().unwrap(),
            category: dependencies.1.pick_random().id().unwrap(),
        }
    }
}
