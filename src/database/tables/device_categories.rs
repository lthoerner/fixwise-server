use proc_macros::{BulkInsert, Relation, IdentifiableRecord, SingleInsert};

use crate::database::{GenerateStaticRecord, GenerateStaticRelation};

#[derive(Relation, BulkInsert, Clone)]
#[relation(
    relation_name = "device_categories",
    primary_key = "id",
    foreign_key_name = "device_category"
)]
pub struct DeviceCategoriesTable {
    records: Vec<DeviceCategoriesTableRecord>,
}

#[derive(SingleInsert, sqlx::FromRow, IdentifiableRecord, Clone)]
pub struct DeviceCategoriesTableRecord {
    pub id: i32,
    pub display_name: String,
}

impl GenerateStaticRelation for DeviceCategoriesTable {
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
