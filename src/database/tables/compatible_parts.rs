use std::collections::HashSet;

use sqlx::query_builder::Separated;
use sqlx::Postgres;

use proc_macros::DatabaseEntity;

use super::device_models::DeviceModelsDatabaseTable;
use super::parts::PartsDatabaseTable;
use super::IdentifiableRow;
use crate::database::{BulkInsert, DatabaseEntity, GenerateRowData, GenerateTableData};

#[derive(DatabaseEntity)]
#[entity(entity_name = "compatible_parts", primary_column = "(device, part)")]
pub struct CompatiblePartsDatabaseJunctionTable {
    rows: Vec<CompatiblePartsDatabaseJunctionTableRow>,
}

impl BulkInsert for CompatiblePartsDatabaseJunctionTable {
    const COLUMN_NAMES: &[&str] = &["device", "part"];
    fn push_bindings(mut builder: Separated<Postgres, &str>, row: Self::Row) {
        builder.push_bind(row.device).push_bind(row.part);
    }
}

#[derive(sqlx::FromRow, Clone)]
pub struct CompatiblePartsDatabaseJunctionTableRow {
    pub device: i32,
    pub part: i32,
}

impl GenerateTableData for CompatiblePartsDatabaseJunctionTable {}
impl GenerateRowData for CompatiblePartsDatabaseJunctionTableRow {
    type Identifier = (i32, i32);
    type Dependencies<'a> = (&'a DeviceModelsDatabaseTable, &'a PartsDatabaseTable);
    fn generate(
        existing_pairs: &mut HashSet<Self::Identifier>,
        dependencies: Self::Dependencies<'_>,
    ) -> Self {
        let mut device_id = 0;
        let mut part_id = 0;
        let mut first_roll = true;
        while first_roll || existing_pairs.get(&(device_id, part_id)).is_some() {
            device_id = dependencies.0.pick_random().id();
            part_id = dependencies.1.pick_random().id();
            first_roll = false;
        }

        existing_pairs.insert((device_id, part_id));

        Self {
            device: device_id,
            part: part_id,
        }
    }
}
