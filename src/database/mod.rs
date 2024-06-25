mod loading_bar;
pub mod shared_models;
pub mod tables;
pub mod views;

use std::sync::Arc;
use std::time::Instant;
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

const TABLE_GENERATION_LOADING_BAR_LENGTH: usize = 33;
const SQL_PARAMETER_BIND_LIMIT: usize = u16::MAX as usize;

const VENDORS_COUNT: usize = 123;
const DEVICE_MANUFACTURERS_COUNT: usize = 123;
const PART_MANUFACTURERS_COUNT: usize = 123;
const DEVICE_MODELS_COUNT: usize = 123;
const DEVICES_COUNT: usize = 1234;
const PARTS_COUNT: usize = 1234;
const CUSTOMERS_COUNT: usize = 1234;
const TICKETS_COUNT: usize = 1234;
const COMPATIBLE_PARTS_COUNT: usize = 1234;
const TICKET_DEVICES_COUNT: usize = 1234;
const BUNDLED_PARTS_COUNT: usize = 1234;

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

    pub async fn add_generated_items(&self) {
        let device_categories = DeviceCategoriesDatabaseTable::generate();
        let part_categories = PartCategoriesDatabaseTable::generate();
        println!("Generating {CUSTOMERS_COUNT} customers");
        let customers = CustomersDatabaseTable::generate(CUSTOMERS_COUNT);
        println!("Generating {VENDORS_COUNT} vendors");
        let vendors = VendorsDatabaseTable::generate(VENDORS_COUNT);
        println!("Generating {DEVICE_MANUFACTURERS_COUNT} device manufacturers");
        let device_manufacturers =
            DeviceManufacturersDatabaseTable::generate(DEVICE_MANUFACTURERS_COUNT);
        println!("Generating {PART_MANUFACTURERS_COUNT} part manufacturers");
        let part_manufacturers = PartManufacturersDatabaseTable::generate(PART_MANUFACTURERS_COUNT);
        println!("Generating {DEVICE_MODELS_COUNT} device models");
        let device_models = DeviceModelsDatabaseTable::generate(
            DEVICE_MODELS_COUNT,
            &device_manufacturers,
            &device_categories,
        );
        println!("Generating {DEVICES_COUNT} devices");
        let devices = DevicesDatabaseTable::generate(DEVICES_COUNT, &device_models, &customers);
        println!("Generating {PARTS_COUNT} parts");
        let parts = PartsDatabaseTable::generate(
            PARTS_COUNT,
            &vendors,
            &part_manufacturers,
            &part_categories,
        );
        println!("Generating {TICKETS_COUNT} tickets");
        let tickets = TicketsDatabaseTable::generate(TICKETS_COUNT, &customers);
        println!("Generating {COMPATIBLE_PARTS_COUNT} compatible parts");
        let compatible_parts = CompatiblePartsDatabaseJunctionTable::generate(
            COMPATIBLE_PARTS_COUNT,
            &device_models,
            &parts,
        );
        println!("Generating {TICKET_DEVICES_COUNT} ticket devices");
        let ticket_devices =
            TicketDevicesDatabaseJunctionTable::generate(TICKET_DEVICES_COUNT, &devices, &tickets);
        println!("Generating {BUNDLED_PARTS_COUNT} bundled parts");
        let bundled_parts = BundledPartsDatabaseJunctionTable::generate(
            BUNDLED_PARTS_COUNT,
            &ticket_devices,
            &parts,
        );

        println!("Inserting items to database...");

        let start_time = Instant::now();

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

        println!(
            "Inserted {} items in {}ms",
            (VENDORS_COUNT
                + DEVICE_MANUFACTURERS_COUNT
                + PART_MANUFACTURERS_COUNT
                + DEVICE_MODELS_COUNT
                + DEVICES_COUNT
                + PARTS_COUNT
                + CUSTOMERS_COUNT
                + TICKETS_COUNT
                + COMPATIBLE_PARTS_COUNT
                + TICKET_DEVICES_COUNT
                + BUNDLED_PARTS_COUNT),
            start_time.elapsed().as_millis()
        );
    }

    async fn execute_querybuilder<'a>(&self, mut querybuilder: QueryBuilder<'a, Postgres>) {
        querybuilder
            .build()
            .execute(&self.connection)
            .await
            .unwrap();
    }
}
