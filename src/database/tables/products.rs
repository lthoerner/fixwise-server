use std::collections::HashSet;

use proc_macros::{
    BulkInsert, GenerateTableData, IdentifiableRecord, Relation, SingleInsert, Table,
};

use super::generators::*;
use crate::database::GenerateRecord;

#[derive(Relation, Table, BulkInsert, GenerateTableData, Clone)]
#[relation(relation_name = "products", primary_key = "sku")]
#[table(foreign_key_name = "product")]
pub struct ProductsTable {
    records: Vec<ProductsTableRecord>,
}

#[derive(SingleInsert, sqlx::FromRow, IdentifiableRecord, Clone)]
pub struct ProductsTableRecord {
    pub sku: i32,
    pub display_name: String,
}

impl GenerateRecord for ProductsTableRecord {
    type Identifier = i32;
    type Dependencies<'a> = ();
    fn generate(
        _existing_records: &[Self],
        existing_ids: &mut HashSet<Self::Identifier>,
        _dependencies: Self::Dependencies<'_>,
    ) -> Self {
        Self {
            sku: generate_unique_i32(0, existing_ids),
            display_name: "PLACEHOLDER".to_owned(),
        }
    }
}
