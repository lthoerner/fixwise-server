use proc_macros::{BulkInsert, DatabaseEntity, IdentifiableRow, SingleInsert};

#[derive(DatabaseEntity, BulkInsert, Clone)]
#[entity(
    schema_name = "persistent",
    entity_name = "type_allocation_codes",
    primary_key = "tac",
    foreign_key_name = "type_allocation_code"
)]
pub struct TypeAllocationCodesDatabaseTable {
    rows: Vec<TypeAllocationCodesDatabaseTableRow>,
}

#[derive(SingleInsert, sqlx::FromRow, IdentifiableRow, Clone, Debug)]
pub struct TypeAllocationCodesDatabaseTableRow {
    pub tac: i32,
    pub manufacturer: String,
    pub model: String,
}
