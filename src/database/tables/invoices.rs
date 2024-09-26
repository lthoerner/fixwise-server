use std::collections::HashSet;

use chrono::NaiveDateTime;

use proc_macros::{
    BulkInsert, GenerateTableData, IdentifiableRecord, Relation, SingleInsert, Table,
};

use super::generators::*;
use crate::database::GenerateRecord;

#[derive(Relation, Table, BulkInsert, GenerateTableData, Clone)]
#[relation(relation_name = "invoices", primary_key = "id")]
pub struct InvoicesTable {
    records: Vec<InvoicesTableRecord>,
}

#[derive(SingleInsert, sqlx::FromRow, IdentifiableRecord, Clone)]
pub struct InvoicesTableRecord {
    pub id: i32,
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
            id: generate_unique_i32(0, existing_ids),
            created_at: Some(created_at),
            updated_at: Some(updated_at),
        }
    }
}
