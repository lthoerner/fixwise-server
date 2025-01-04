use proc_macros::{
    BulkInsert, IdentifiableRecord, ReadRecord, ReadRelation, Record, Relation, SingleInsert,
    WriteRecord, WriteRelation,
};

#[derive(Relation, ReadRelation, WriteRelation, BulkInsert, Clone)]
#[relation(
    schema_name = "persistent",
    relation_name = "type_allocation_codes",
    primary_key = "tac"
)]
pub struct TypeAllocationCodesTable {
    records: Vec<TypeAllocationCodesTableRecord>,
}

#[derive(
    Record, ReadRecord, WriteRecord, SingleInsert, sqlx::FromRow, IdentifiableRecord, Clone, Debug,
)]
pub struct TypeAllocationCodesTableRecord {
    #[manual_primary_key]
    pub tac: i32,
    pub manufacturer: String,
    pub model: String,
}
