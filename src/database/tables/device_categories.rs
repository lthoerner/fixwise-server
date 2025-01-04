use proc_macros::{
    BulkInsert, IdentifiableRecord, ReadRecord, ReadRelation, Record, Relation, SingleInsert,
    WriteRecord, WriteRelation,
};

use crate::database::{GenerateStaticRecord, GenerateStaticTable};

#[derive(Relation, ReadRelation, WriteRelation, BulkInsert, Clone)]
#[relation(relation_name = "device_categories", primary_key = "id")]
pub struct DeviceCategoriesTable {
    records: Vec<DeviceCategoriesTableRecord>,
}

#[derive(
    Record, ReadRecord, WriteRecord, SingleInsert, sqlx::FromRow, IdentifiableRecord, Clone,
)]
pub struct DeviceCategoriesTableRecord {
    #[auto_primary_key]
    pub id: i32,
    pub display_name: String,
}

impl GenerateStaticTable for DeviceCategoriesTable {
    const ITEMS: &[&str] = &[
        "Phone",
        "Tablet",
        "Desktop",
        "Laptop",
        "Game Console",
        "Camera",
        "Drone",
    ];
}

impl GenerateStaticRecord for DeviceCategoriesTableRecord {
    fn new(id: i32, display_name: impl Into<String>) -> Self {
        Self {
            id,
            display_name: display_name.into(),
        }
    }
}
