use std::collections::HashSet;

use proc_macros::{BulkInsert, Relation, GenerateTableData, IdentifiableRecord, SingleInsert};

use super::customers::CustomersTable;
use super::device_models::DeviceModelsTable;
use super::generators::*;
use super::IdentifiableRecord;
use crate::database::{GenerateRecord, Relation};

#[derive(Relation, BulkInsert, GenerateTableData, Clone)]
#[relation(
    relation_name = "devices",
    primary_key = "id",
    foreign_key_name = "device"
)]
pub struct DevicesTable {
    records: Vec<DevicesTableRecord>,
}

#[derive(SingleInsert, sqlx::FromRow, IdentifiableRecord, Clone)]
pub struct DevicesTableRecord {
    pub id: i32,
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
            id: generate_unique_i32(0, existing_ids),
            model: dependencies.0.pick_random().id(),
            owner: generate_option(dependencies.1.pick_random().id(), 0.9),
        }
    }
}
