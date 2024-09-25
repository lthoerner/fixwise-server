use std::collections::HashSet;

use proc_macros::{BulkInsert, GenerateTableData, Relation, SingleInsert};

use super::device_models::DeviceModelsTable;
use super::parts::PartsTable;
use super::IdentifiableRow;
use crate::database::{GenerateRecord, Relation};

#[derive(Relation, BulkInsert, GenerateTableData, Clone)]
#[relation(
    relation_name = "compatible_parts",
    primary_key = "(device, part)",
    foreign_key_name = "compatible_part"
)]
pub struct CompatiblePartsJunctionTable {
    records: Vec<CompatiblePartsJunctionTableRecord>,
}

#[derive(SingleInsert, sqlx::FromRow, Clone)]
pub struct CompatiblePartsJunctionTableRecord {
    pub device: i32,
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
