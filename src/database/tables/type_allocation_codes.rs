use crudkit::prelude::*;
use serde::Serialize;

#[derive(Relation, ReadRelation, WriteRelation, BulkInsert, Serialize, Clone)]
#[relation(
    schema_name = "persistent",
    relation_name = "type_allocation_codes",
    primary_key = "tac"
)]
pub struct TypeAllocationCodesTable {
    records: Vec<TypeAllocationCodesTableRecord>,
}

#[derive(Record, ReadRecord, WriteRecord, SingleInsert, Serialize, sqlx::FromRow, Clone, Debug)]
pub struct TypeAllocationCodesTableRecord {
    #[manual_primary_key]
    pub tac: i32,
    pub manufacturer: String,
    pub model: String,
}
