use std::collections::HashSet;

use serde::Deserialize;

use crudkit::{
    BulkInsert, IdParameter, IdentifiableRecord, ReadRecord, ReadRelation, Record, Relation,
    SingleInsert, WriteRecord, WriteRelation,
};
use serde::Serialize;

use proc_macros::GenerateTable;

use super::generators::*;
use crate::database::traits::GenerateRecord;

#[derive(Clone, Deserialize, IdParameter)]
pub struct SkuParameter {
    sku: usize,
}

#[derive(Relation, ReadRelation, WriteRelation, BulkInsert, GenerateTable, Serialize, Clone)]
#[relation(relation_name = "products", primary_key = "sku")]
pub struct ProductsTable {
    records: Vec<ProductsTableRecord>,
}

#[derive(
    Record,
    ReadRecord,
    WriteRecord,
    SingleInsert,
    Serialize,
    sqlx::FromRow,
    IdentifiableRecord,
    Clone,
)]
pub struct ProductsTableRecord {
    #[auto_primary_key]
    #[defaultable]
    pub sku: Option<i32>,
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
            sku: Some(generate_unique_i32(0, existing_ids)),
            display_name: "PLACEHOLDER".to_owned(),
        }
    }
}
