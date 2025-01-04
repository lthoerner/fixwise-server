use proc_macros::{
    BulkInsert, IdentifiableRecord, ReadRecord, ReadRelation, Record, Relation, SingleInsert,
    WriteRecord, WriteRelation,
};

use crate::database::{GenerateStaticRecord, GenerateStaticTable};

#[derive(Relation, ReadRelation, WriteRelation, BulkInsert, Clone)]
#[relation(relation_name = "part_categories", primary_key = "id")]
pub struct PartCategoriesTable {
    records: Vec<PartCategoriesTableRecord>,
}

#[derive(
    Record, ReadRecord, WriteRecord, SingleInsert, sqlx::FromRow, IdentifiableRecord, Clone,
)]
pub struct PartCategoriesTableRecord {
    #[auto_primary_key]
    pub id: i32,
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
            id,
            display_name: display_name.into(),
        }
    }
}
