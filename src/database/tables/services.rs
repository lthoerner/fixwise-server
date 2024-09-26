use std::collections::HashSet;

use proc_macros::{
    BulkInsert, GenerateTableData, IdentifiableRecord, Relation, SingleInsert, Table,
};

use super::device_models::DeviceModelsTable;
use super::generators::*;
use super::service_types::ServiceTypesTable;
use super::IdentifiableRecord;
use crate::database::{GenerateRecord, Relation};

#[derive(Relation, Table, BulkInsert, GenerateTableData, Clone)]
#[relation(relation_name = "services", primary_key = "id")]
pub struct ServicesTable {
    records: Vec<ServicesTableRecord>,
}

#[derive(SingleInsert, sqlx::FromRow, IdentifiableRecord, Clone)]
pub struct ServicesTableRecord {
    pub id: i32,
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
            id: generate_unique_i32(0, existing_ids),
            r#type: dependencies.0.pick_random().id(),
            device: dependencies.1.pick_random().id(),
        }
    }
}
