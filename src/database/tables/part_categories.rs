use crudkit::{
    BulkInsert, IdentifiableRecord, ReadRecord, ReadRelation, Record, Relation, SingleInsert,
    WriteRecord, WriteRelation,
};
use serde::Serialize;

use crate::database::{GenerateStaticRecord, GenerateStaticTable};

#[derive(Relation, ReadRelation, WriteRelation, BulkInsert, Serialize, Clone)]
#[relation(relation_name = "part_categories", primary_key = "id")]
pub struct PartCategoriesTable {
    records: Vec<PartCategoriesTableRecord>,
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
pub struct PartCategoriesTableRecord {
    #[auto_primary_key]
    #[defaultable]
    pub id: Option<i32>,
    pub display_name: String,
}

impl GenerateStaticTable for PartCategoriesTable {
    const ITEMS: &[&str] = &[
        "Screen",
        "Battery",
        "Backglass",
        "Frame",
        "Front Camera",
        "Rear Camera",
        "Charge Port",
    ];
}

impl GenerateStaticRecord for PartCategoriesTableRecord {
    fn new(id: i32, display_name: impl Into<String>) -> Self {
        Self {
            id: Some(id),
            display_name: display_name.into(),
        }
    }
}
