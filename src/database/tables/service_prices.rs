use std::collections::HashSet;

use chrono::NaiveDateTime;
use rust_decimal::Decimal;

use crudkit::traits::shared::{IdentifiableRecord, Relation};
use crudkit::{
    BulkInsert, IdentifiableRecord, ReadRecord, ReadRelation, Record, Relation, SingleInsert,
    WriteRecord, WriteRelation,
};
use serde::Serialize;

use proc_macros::GenerateTable;

use super::generators::*;
use super::services::ServicesTable;
use crate::database::traits::GenerateRecord;

#[derive(Relation, ReadRelation, WriteRelation, BulkInsert, GenerateTable, Serialize, Clone)]
#[relation(relation_name = "service_prices", primary_key = "id")]
pub struct ServicePricesTable {
    records: Vec<ServicePricesTableRecord>,
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
pub struct ServicePricesTableRecord {
    #[auto_primary_key]
    #[defaultable]
    pub id: Option<i32>,
    pub service: i32,
    #[defaultable]
    pub base_fee: Option<Decimal>,
    #[defaultable]
    pub labor_fee: Option<Decimal>,
    #[defaultable]
    pub time_set: Option<NaiveDateTime>,
}

impl GenerateRecord for ServicePricesTableRecord {
    type Identifier = i32;
    type Dependencies<'a> = &'a ServicesTable;
    fn generate(
        _existing_records: &[Self],
        existing_ids: &mut HashSet<Self::Identifier>,
        dependencies: Self::Dependencies<'_>,
    ) -> Self {
        let base_fee = generate_dollar_value(Some(1.00), Some(500.00));
        let labor_fee = generate_dollar_value(Some(1.00), Some(500.00));

        Self {
            id: Some(generate_unique_i32(0, existing_ids)),
            service: dependencies.pick_random().id().unwrap(),
            base_fee: Some(base_fee),
            labor_fee: Some(labor_fee),
            time_set: Some(generate_date(None)),
        }
    }
}
