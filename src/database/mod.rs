mod loading_bar;
pub mod shared_models;
pub mod tables;
pub mod views;

use std::collections::HashSet;
use std::sync::Arc;
use std::time::Instant;

use axum::extract::{Query, State};

use rand::{thread_rng, Rng};
use serde::Deserialize;
use sqlx::postgres::PgRow;
use sqlx::query_builder::{QueryBuilder, Separated};
use sqlx::{raw_sql, PgPool, Postgres};

use proc_macros::IdParameter;

use crate::ServerState;
use loading_bar::LoadingBar;
use tables::bundled_parts::BundledPartsDatabaseJunctionTable;
use tables::compatible_parts::CompatiblePartsDatabaseJunctionTable;
use tables::customers::CustomersDatabaseTable;
use tables::device_categories::DeviceCategoriesDatabaseTable;
use tables::device_manufacturers::DeviceManufacturersDatabaseTable;
use tables::device_models::DeviceModelsDatabaseTable;
use tables::devices::DevicesDatabaseTable;
use tables::generators::*;
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
    connection: PgPool,
}

// ? Should this be accompanied by a `DatabaseRow` trait?
pub trait DatabaseEntity: Sized {
    type Row: for<'a> sqlx::FromRow<'a, PgRow> + Send + Unpin + Clone;
    const SCHEMA_NAME: &str = "main";
    const ENTITY_NAME: &str;
    const PRIMARY_COLUMN_NAME: &str;

    // TODO: Take `Into<Vec<Self::Row>>` here
    fn with_rows(rows: Vec<Self::Row>) -> Self;
    fn take_rows(self) -> Vec<Self::Row>;
    fn rows(&self) -> &[Self::Row];

    async fn query_one(
        State(state): State<Arc<ServerState>>,
        id_param: Query<impl IdParameter>,
    ) -> Option<Self::Row> {
        sqlx::query_as(&format!(
            "SELECT * FROM {}.{} WHERE {} = {}",
            Self::SCHEMA_NAME,
            Self::ENTITY_NAME,
            Self::PRIMARY_COLUMN_NAME,
            id_param.id(),
        ))
        .fetch_one(&state.database.connection)
        .await
        .ok()
    }

    async fn query_all(State(state): State<Arc<ServerState>>) -> Self {
        Self::with_rows(
            sqlx::query_as(&format!(
                "SELECT * FROM {}.{} ORDER BY {}",
                Self::SCHEMA_NAME,
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

trait GenerateTableData: DatabaseEntity<Row: GenerateRowData> {
    fn generate(
        count: usize,
        dependencies: <<Self as DatabaseEntity>::Row as GenerateRowData>::Dependencies<'_>,
    ) -> Self {
        let mut rows = Vec::new();
        let mut existing_ids = HashSet::new();
        let mut loading_bar = LoadingBar::new(count);
        for _ in 0..count {
            loading_bar.update();
            rows.push(<Self as DatabaseEntity>::Row::generate(
                &mut existing_ids,
                dependencies,
            ))
        }

        Self::with_rows(rows)
    }
}

trait GenerateRowData {
    type Identifier: Copy;
    type Dependencies<'a>: Copy;
    fn generate(
        existing_ids: &mut HashSet<Self::Identifier>,
        dependencies: Self::Dependencies<'_>,
    ) -> Self;
}

trait GenerateStaticTableData: DatabaseEntity<Row: GenerateStaticRowData> {
    const ITEMS: &[&str];
    fn generate() -> Self {
        let mut existing_ids = HashSet::new();
        let rows = Self::ITEMS
            .iter()
            .map(|item| {
                Self::Row::new(
                    generate_unique_i32(0, &mut existing_ids),
                    (*item).to_owned(),
                )
            })
            .collect();

        Self::with_rows(rows)
    }
}

trait GenerateStaticRowData {
    fn new(id: i32, display_name: String) -> Self;
}

// TODO: Add standard single-insert trait
pub trait BulkInsert: DatabaseEntity {
    const COLUMN_NAMES: &[&str];
    const CHUNK_SIZE: usize = SQL_PARAMETER_BIND_LIMIT / Self::COLUMN_NAMES.len();

    fn get_querybuilder<'a>() -> QueryBuilder<'a, Postgres> {
        QueryBuilder::new(&format!(
            "INSERT INTO {}.{} ({}) ",
            Self::SCHEMA_NAME,
            Self::ENTITY_NAME,
            Self::COLUMN_NAMES.join(", ")
        ))
    }

    fn into_chunks(self) -> impl Iterator<Item = Vec<Self::Row>> {
        let mut iter = self.take_rows().into_iter();
        // TODO: Annotate this code or something, I have very little idea what it does
        // * This was done because `itertools::IntoChunks` was causing issues with the axum handlers
        std::iter::from_fn(move || Some(iter.by_ref().take(Self::CHUNK_SIZE).collect()))
            .take_while(|v: &Vec<_>| v.len() > 0)
    }

    fn push_bindings(builder: Separated<Postgres, &str>, row: Self::Row);

    async fn insert_all(self, database: &Database) {
        for chunk in self.into_chunks() {
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

    pub async fn close_connection(&self) {
        self.connection.close().await
    }

    pub async fn add_generated_items(&self) {
        let device_categories = DeviceCategoriesDatabaseTable::generate();
        let part_categories = PartCategoriesDatabaseTable::generate();
        println!("Generating {CUSTOMERS_COUNT} customers");
        let customers = CustomersDatabaseTable::generate(CUSTOMERS_COUNT, ());
        println!("Generating {VENDORS_COUNT} vendors");
        let vendors = VendorsDatabaseTable::generate(VENDORS_COUNT, ());
        println!("Generating {DEVICE_MANUFACTURERS_COUNT} device manufacturers");
        let device_manufacturers =
            DeviceManufacturersDatabaseTable::generate(DEVICE_MANUFACTURERS_COUNT, ());
        println!("Generating {PART_MANUFACTURERS_COUNT} part manufacturers");
        let part_manufacturers =
            PartManufacturersDatabaseTable::generate(PART_MANUFACTURERS_COUNT, ());
        println!("Generating {DEVICE_MODELS_COUNT} device models");
        let device_models = DeviceModelsDatabaseTable::generate(
            DEVICE_MODELS_COUNT,
            (&device_manufacturers, &device_categories),
        );
        println!("Generating {DEVICES_COUNT} devices");
        let devices = DevicesDatabaseTable::generate(DEVICES_COUNT, (&device_models, &customers));
        println!("Generating {PARTS_COUNT} parts");
        let parts = PartsDatabaseTable::generate(
            PARTS_COUNT,
            (&vendors, &part_manufacturers, &part_categories),
        );
        println!("Generating {TICKETS_COUNT} tickets");
        let tickets = TicketsDatabaseTable::generate(TICKETS_COUNT, &customers);
        println!("Generating {COMPATIBLE_PARTS_COUNT} compatible parts");
        let compatible_parts = CompatiblePartsDatabaseJunctionTable::generate(
            COMPATIBLE_PARTS_COUNT,
            (&device_models, &parts),
        );
        println!("Generating {TICKET_DEVICES_COUNT} ticket devices");
        let ticket_devices = TicketDevicesDatabaseJunctionTable::generate(
            TICKET_DEVICES_COUNT,
            (&tickets, &devices),
        );
        println!("Generating {BUNDLED_PARTS_COUNT} bundled parts");
        let bundled_parts = BundledPartsDatabaseJunctionTable::generate(
            BUNDLED_PARTS_COUNT,
            (&ticket_devices, &parts),
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

    // TODO: Move this logic to `BulkInsert::insert_all()`
    async fn execute_querybuilder<'a>(&self, mut querybuilder: QueryBuilder<'a, Postgres>) {
        querybuilder
            .build()
            .execute(&self.connection)
            .await
            .unwrap();
    }
}

pub trait IdParameter {
    fn new(value: usize) -> Self;
    fn id(&self) -> usize;
}

#[derive(Clone, Deserialize, IdParameter)]
pub struct GenericIdParameter {
    id: usize,
}
