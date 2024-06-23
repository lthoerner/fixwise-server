pub mod shared_models;
pub mod tables;
pub mod views;

use std::sync::Arc;
use std::vec::IntoIter;

use axum::extract::State;
use itertools::{IntoChunks, Itertools};
use rand::{thread_rng, Rng};
use sqlx::postgres::PgRow;
use sqlx::query_builder::{QueryBuilder, Separated};
use sqlx::{raw_sql, PgPool, Postgres};

use crate::ServerState;
use tables::bundled_parts::BundledPartsDatabaseJunctionTable;
use tables::compatible_parts::CompatiblePartsDatabaseJunctionTable;
use tables::customers::CustomersDatabaseTable;
use tables::device_categories::DeviceCategoriesDatabaseTable;
use tables::device_manufacturers::DeviceManufacturersDatabaseTable;
use tables::device_models::DeviceModelsDatabaseTable;
use tables::devices::DevicesDatabaseTable;
use tables::part_categories::PartCategoriesDatabaseTable;
use tables::part_manufacturers::PartManufacturersDatabaseTable;
use tables::parts::PartsDatabaseTable;
use tables::ticket_devices::TicketDevicesDatabaseJunctionTable;
use tables::tickets::TicketsDatabaseTable;
use tables::vendors::VendorsDatabaseTable;

const SQL_PARAMETER_BIND_LIMIT: usize = u16::MAX as usize;

#[derive(Clone)]
pub struct Database {
    pub connection: PgPool,
}

pub trait DatabaseEntity: Sized {
    type Row: for<'a> sqlx::FromRow<'a, PgRow> + Send + Unpin + Clone;
    const ENTITY_NAME: &str;
    const PRIMARY_COLUMN_NAME: &str;

    fn with_rows(rows: Vec<Self::Row>) -> Self;
    fn take_rows(self) -> Vec<Self::Row>;
    fn rows(&self) -> &[Self::Row];

    async fn query_all(State(state): State<Arc<ServerState>>) -> Self {
        Self::with_rows(
            sqlx::query_as(&format!(
                "SELECT * FROM main.{} ORDER BY {}",
                Self::ENTITY_NAME,
                Self::PRIMARY_COLUMN_NAME
            ))
            .fetch_all(&state.database.connection)
            .await
            .unwrap(),
        )
    }

    fn pick_random(&self) -> Self::Row {
        let rows = self.rows();
        rows[thread_rng().gen_range(0..rows.len())].clone()
    }
}

pub trait BulkInsert: DatabaseEntity {
    const COLUMN_NAMES: &[&str];
    const CHUNK_SIZE: usize = SQL_PARAMETER_BIND_LIMIT / Self::COLUMN_NAMES.len();

    fn get_querybuilder<'a>() -> QueryBuilder<'a, Postgres> {
        QueryBuilder::new(&format!(
            "INSERT INTO main.{} ({}) ",
            Self::ENTITY_NAME,
            Self::COLUMN_NAMES.join(", ")
        ))
    }

    fn into_chunks(self) -> IntoChunks<IntoIter<Self::Row>> {
        let num_chunks = usize::div_ceil(self.rows().len(), Self::CHUNK_SIZE);
        self.take_rows().into_iter().chunks(num_chunks)
    }

    fn push_bindings(builder: Separated<Postgres, &str>, row: Self::Row);

    async fn insert_all(self, database: &Database) {
        for chunk in &self.into_chunks() {
            let mut querybuilder = Self::get_querybuilder();
            querybuilder.push_values(chunk, Self::push_bindings);
            database.execute_querybuilder(querybuilder).await;
        }
    }
}

impl Database {
    const CONFIG_SCRIPT: &str = include_str!("../../database/config.sql");

    pub async fn connect_and_configure() -> Self {
        let database = Self::connect().await;
        database.configure().await;

        database
    }

    async fn connect() -> Self {
        Self {
            connection: PgPool::connect("postgresql://techtriage:techtriage@localhost:5432")
                .await
                .unwrap(),
        }
    }

    async fn configure(&self) {
        raw_sql(Self::CONFIG_SCRIPT)
            .execute(&self.connection)
            .await
            .unwrap();
    }

    pub async fn add_items(
        &self,
        bundled_parts: BundledPartsDatabaseJunctionTable,
        compatible_parts: CompatiblePartsDatabaseJunctionTable,
        customers: CustomersDatabaseTable,
        device_categories: DeviceCategoriesDatabaseTable,
        device_manufacturers: DeviceManufacturersDatabaseTable,
        device_models: DeviceModelsDatabaseTable,
        devices: DevicesDatabaseTable,
        part_categories: PartCategoriesDatabaseTable,
        part_manufacturers: PartManufacturersDatabaseTable,
        parts: PartsDatabaseTable,
        ticket_devices: TicketDevicesDatabaseJunctionTable,
        tickets: TicketsDatabaseTable,
        vendors: VendorsDatabaseTable,
    ) {
        vendors.insert_all(self).await;
        device_manufacturers.insert_all(self).await;
        part_manufacturers.insert_all(self).await;
        device_categories.insert_all(self).await;
        part_categories.insert_all(self).await;
        device_models.insert_all(self).await;
        parts.insert_all(self).await;
        customers.insert_all(self).await;
        devices.insert_all(self).await;
        tickets.insert_all(self).await;
        compatible_parts.insert_all(self).await;
        ticket_devices.insert_all(self).await;
        bundled_parts.insert_all(self).await;
    }

    async fn execute_querybuilder<'a>(&self, mut querybuilder: QueryBuilder<'a, Postgres>) {
        querybuilder
            .build()
            .execute(&self.connection)
            .await
            .unwrap();
    }
}
