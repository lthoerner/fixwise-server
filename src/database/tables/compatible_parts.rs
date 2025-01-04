use std::collections::HashSet;

use proc_macros::{
    BulkInsert, CreateAndUpdate, GenerateTable, ReadRelation, Relation, SingleInsert, WriteRelation,
};

use super::device_models::DeviceModelsTable;
use super::parts::PartsTable;
use super::IdentifiableRecord;
use crate::database::traits::generate::GenerateRecord;
use crate::database::traits::shared::Relation;

#[derive(Relation, ReadRelation, WriteRelation, BulkInsert, GenerateTable, Clone)]
#[relation(relation_name = "compatible_parts", primary_key = "(device, part)")]
pub struct CompatiblePartsJunctionTable {
    records: Vec<CompatiblePartsJunctionTableRecord>,
}

#[derive(SingleInsert, CreateAndUpdate, sqlx::FromRow, Clone)]
pub struct CompatiblePartsJunctionTableRecord {
    #[manual_primary_key]
    pub device: i32,
    #[manual_primary_key]
    pub part: i32,
}

impl GenerateRecord for CompatiblePartsJunctionTableRecord {
    type Identifier = (i32, i32);
    type Dependencies<'a> = (&'a DeviceModelsTable, &'a PartsTable);
    fn generate(
        _existing_records: &[Self],
        existing_pairs: &mut HashSet<Self::Identifier>,
        dependencies: Self::Dependencies<'_>,
    ) -> Self {
        let mut device = 0;
        let mut part = 0;
        let mut first_roll = true;
        while first_roll || existing_pairs.get(&(device, part)).is_some() {
            device = dependencies.0.pick_random().id();
            part = dependencies.1.pick_random().id();
            first_roll = false;
        }

        existing_pairs.insert((device, part));

        Self { device, part }
    }
}
