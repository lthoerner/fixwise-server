use crudkit::prelude::*;
use serde::Serialize;

use crate::database::{GenerateStaticRecord, GenerateStaticTable};

#[derive(Relation, ReadRelation, WriteRelation, BulkInsert, Serialize, Clone)]
#[relation(relation_name = "service_types", primary_key = "id")]
pub struct ServiceTypesTable {
    records: Vec<ServiceTypesTableRecord>,
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
pub struct ServiceTypesTableRecord {
    #[auto_primary_key]
    #[defaultable]
    pub id: Option<i32>,
    pub display_name: String,
}

impl GenerateStaticTable for ServiceTypesTable {
    const ITEMS: &[&str] = &[
        "Screen Repair",
        "Battery Repair",
        "Backglass Repair",
        "Camera Repair",
        "Port Repair",
        "Other Repair",
    ];
}

impl GenerateStaticRecord for ServiceTypesTableRecord {
    fn new(id: i32, display_name: impl Into<String>) -> Self {
        Self {
            id: Some(id),
            display_name: display_name.into(),
        }
    }
}
