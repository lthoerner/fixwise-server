use std::collections::HashSet;

use sqlx::query_builder::Separated;
use sqlx::Postgres;

use super::generators::*;
use super::IdentifiableRow;
use crate::database::{BulkInsert, DatabaseEntity};

pub struct CustomersDatabaseTable {
    rows: Vec<CustomersDatabaseTableRow>,
}

impl DatabaseEntity for CustomersDatabaseTable {
    type Row = CustomersDatabaseTableRow;
    const ENTITY_NAME: &str = "customers";
    const PRIMARY_COLUMN_NAME: &str = "id";

    fn with_rows(rows: Vec<Self::Row>) -> Self {
        Self { rows }
    }

    fn take_rows(self) -> Vec<Self::Row> {
        self.rows
    }

    fn rows(&self) -> &[Self::Row] {
        &self.rows
    }
}

impl BulkInsert for CustomersDatabaseTable {
    const COLUMN_NAMES: &[&str] = &[
        "id",
        "name",
        "email_address",
        "phone_number",
        "street_address",
    ];

    fn push_bindings(mut builder: Separated<Postgres, &str>, row: Self::Row) {
        builder
            .push_bind(row.id)
            .push_bind(row.name)
            .push_bind(row.email_address)
            .push_bind(row.phone_number)
            .push_bind(row.street_address);
    }
}

#[derive(sqlx::FromRow, Clone)]
pub struct CustomersDatabaseTableRow {
    pub id: i32,
    pub name: String,
    pub email_address: Option<String>,
    pub phone_number: Option<String>,
    pub street_address: Option<String>,
}

impl IdentifiableRow for CustomersDatabaseTableRow {
    fn id(&self) -> i32 {
        self.id
    }
}

impl CustomersDatabaseTable {
    fn generate(count: usize) -> Self {
        let mut rows = Vec::new();
        let mut existing_ids = HashSet::new();
        for _ in 0..count {
            rows.push(CustomersDatabaseTableRow::generate(&mut existing_ids));
        }

        Self::with_rows(rows)
    }
}

impl CustomersDatabaseTableRow {
    fn generate(existing_ids: &mut HashSet<i32>) -> Self {
        Self {
            id: generate_unique_i32(0, existing_ids),
            name: generate_name(),
            email_address: Some(generate_email_address()),
            phone_number: Some(generate_phone_number()),
            street_address: Some(generate_address()),
        }
    }
}
