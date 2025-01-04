use proc_macros::{
    BulkInsert, CreateAndUpdate, IdentifiableRecord, ReadRelation, Relation, SingleInsert, Table,
};

#[derive(Relation, ReadRelation, Table, BulkInsert, Clone)]
#[relation(
    schema_name = "persistent",
    relation_name = "type_allocation_codes",
    primary_key = "tac"
)]
pub struct TypeAllocationCodesTable {
    records: Vec<TypeAllocationCodesTableRecord>,
}

#[derive(SingleInsert, CreateAndUpdate, sqlx::FromRow, IdentifiableRecord, Clone, Debug)]
pub struct TypeAllocationCodesTableRecord {
    #[manual_primary_key]
    pub tac: i32,
    pub manufacturer: String,
    pub model: String,
}
