use std::collections::HashSet;

use chrono::NaiveDateTime;
use rust_decimal::Decimal;

use proc_macros::{BulkInsert, DatabaseEntity, GenerateTableData, IdentifiableRow, SingleInsert};

use super::generators::*;
use super::services::ServicesDatabaseTable;
use super::IdentifiableRow;
use crate::database::{DatabaseEntity, GenerateRowData};

#[derive(DatabaseEntity, BulkInsert, GenerateTableData, Clone)]
#[entity(entity_name = "Service_prices", primary_key = "id")]
pub struct ServicePricesDatabaseTable {
    rows: Vec<ServicePricesDatabaseTableRow>,
}

#[derive(SingleInsert, sqlx::FromRow, IdentifiableRow, Clone)]
pub struct ServicePricesDatabaseTableRow {
    pub id: i32,
    pub service: i32,
    #[defaultable]
    pub base_fee: Option<Decimal>,
    #[defaultable]
    pub labor_fee: Option<Decimal>,
    #[defaultable]
    pub time_set: Option<NaiveDateTime>,
}

impl GenerateRowData for ServicePricesDatabaseTableRow {
    type Identifier = i32;
    type Dependencies<'a> = &'a ServicesDatabaseTable;
    fn generate(
        _existing_rows: &[Self],
        existing_ids: &mut HashSet<Self::Identifier>,
        dependencies: Self::Dependencies<'_>,
    ) -> Self {
        let base_fee = generate_dollar_value(Some(1.00), Some(500.00));
        let labor_fee = generate_dollar_value(Some(1.00), Some(500.00));

        Self {
            id: generate_unique_i32(0, existing_ids),
            service: dependencies.pick_random().id(),
            base_fee: Some(base_fee),
            labor_fee: Some(labor_fee),
            time_set: Some(generate_date(None)),
        }
    }
}
