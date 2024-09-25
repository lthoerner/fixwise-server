use proc_macros::{BulkInsert, Relation, IdentifiableRow, SingleInsert};

#[derive(Relation, BulkInsert, Clone)]
#[relation(
    schema_name = "persistent",
    relation_name = "type_allocation_codes",
    primary_key = "tac",
    foreign_key_name = "type_allocation_code"
)]
pub struct TypeAllocationCodesTable {
    records: Vec<TypeAllocationCodesTableRecord>,
}

#[derive(SingleInsert, sqlx::FromRow, IdentifiableRow, Clone, Debug)]
pub struct TypeAllocationCodesTableRecord {
    pub tac: i32,
    pub manufacturer: String,
    pub model: String,
}
