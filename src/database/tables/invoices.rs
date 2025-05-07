use std::collections::HashSet;

use chrono::NaiveDateTime;

use crudkit::prelude::*;
use serde::Serialize;

use proc_macros::GenerateTable;

use super::generators::*;
use crate::database::traits::GenerateRecord;

#[derive(Relation, ReadRelation, WriteRelation, BulkInsert, GenerateTable, Serialize, Clone)]
#[relation(relation_name = "invoices", primary_key = "id")]
pub struct InvoicesTable {
    records: Vec<InvoicesTableRecord>,
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
pub struct InvoicesTableRecord {
    #[auto_primary_key]
    pub id: Option<i32>,
    #[defaultable]
    pub created_at: Option<NaiveDateTime>,
    #[defaultable]
    pub updated_at: Option<NaiveDateTime>,
}

impl GenerateRecord for InvoicesTableRecord {
    type Identifier = i32;
    type Dependencies<'a> = ();
    fn generate(
        _existing_records: &[Self],
        existing_ids: &mut HashSet<Self::Identifier>,
        _dependencies: Self::Dependencies<'_>,
    ) -> Self {
        let created_at = generate_date(None);
        let updated_at = generate_date(Some(created_at));

        Self {
            id: Some(generate_unique_i32(0, existing_ids)),
            created_at: Some(created_at),
            updated_at: Some(updated_at),
        }
    }
}
