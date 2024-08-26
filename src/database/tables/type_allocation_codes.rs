use proc_macros::{BulkInsert, DatabaseEntity, IdentifiableRow};

#[derive(DatabaseEntity, BulkInsert)]
#[entity(
    schema_name = "persistent",
    entity_name = "type_allocation_codes",
    primary_column = "tac",
    columns = ["tac", "manufacturer", "model"]
)]
pub struct TypeAllocationCodesDatabaseTable {
    rows: Vec<TypeAllocationCodesDatabaseTableRow>,
}

#[derive(sqlx::FromRow, Clone, Debug, IdentifiableRow)]
pub struct TypeAllocationCodesDatabaseTableRow {
    pub tac: i32,
    pub manufacturer: String,
    pub model: String,
}
