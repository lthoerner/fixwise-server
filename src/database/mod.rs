mod loading_bar;
pub mod shared_models;
pub mod tables;
pub mod views;

use std::collections::HashSet;
use std::sync::Arc;
use std::time::Instant;

use axum::extract::{Query, State};

use rand::{thread_rng, Rng};
use sqlx::postgres::PgRow;
use sqlx::query_builder::{QueryBuilder, Separated};
use sqlx::{raw_sql, PgPool, Postgres};

use crate::api::IdParameter;
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
use tables::invoice_items::InvoiceItemsDatabaseTable;
use tables::invoice_payments::InvoicePaymentsDatabaseTable;
use tables::invoices::InvoicesDatabaseTable;
use tables::items::ItemsDatabaseTable;
use tables::part_categories::PartCategoriesDatabaseTable;
use tables::part_manufacturers::PartManufacturersDatabaseTable;
use tables::parts::PartsDatabaseTable;
use tables::product_prices::ProductPricesDatabaseTable;
use tables::products::ProductsDatabaseTable;
use tables::service_prices::ServicePricesDatabaseTable;
use tables::service_types::ServiceTypesDatabaseTable;
use tables::services::ServicesDatabaseTable;
use tables::ticket_devices::TicketDevicesDatabaseJunctionTable;
use tables::tickets::TicketsDatabaseTable;
use tables::vendors::VendorsDatabaseTable;

const TABLE_GENERATION_LOADING_BAR_LENGTH: usize = 33;
const SQL_PARAMETER_BIND_LIMIT: usize = u16::MAX as usize;

const VENDORS_COUNT: usize = 123;
const DEVICE_MANUFACTURERS_COUNT: usize = 123;
const PART_MANUFACTURERS_COUNT: usize = 123;
const DEVICE_MODELS_COUNT: usize = 123;
const PARTS_COUNT: usize = 1234;
const PRODUCTS_COUNT: usize = 1234;
const PRODUCT_PRICES_COUNT: usize = 1234;
const SERVICES_COUNT: usize = 1234;
const SERVICE_PRICES_COUNT: usize = 1234;
const CUSTOMERS_COUNT: usize = 1234;
const DEVICES_COUNT: usize = 1234;
const INVOICES_COUNT: usize = 1234;
const INVOICE_ITEMS_COUNT: usize = 1234;
const INVOICE_PAYMENTS_COUNT: usize = 123;
const TICKETS_COUNT: usize = 1234;
const COMPATIBLE_PARTS_COUNT: usize = 1234;
const TICKET_DEVICES_COUNT: usize = 1234;
const BUNDLED_PARTS_COUNT: usize = 1234;

#[derive(Clone)]
pub struct Database {
    connection: PgPool,
}

/// A trait that allows table and view types to interoperate with and be queried from the database.
///
/// This does not implement any insertion methods because "entities" can be views, which are
/// read-only. For inserting items to tables, see the [`SingleInsert`] and [`BulkInsert`] traits.
///
/// This trait does not do a lot on its own but it, along with [`DatabaseRow`], provides the
/// functionality which allows almost all of the other database traits to be auto-implemented or
/// conveniently derived.
pub trait DatabaseEntity: Sized {
    /// The row type which this entity contains a collection of.
    ///
    /// This type and the [`DatabaseRow::Entity`] type are directly interreferential to allow
    /// "upcasting" and "downcasting," mostly for auto-implementations in other traits.
    type Row: DatabaseRow<Entity = Self>;

    /// The name of the schema in which this entity exists in the database.
    ///
    /// This defaults to "main" but can be changed in case an entity lives in a different schema.
    /// The main alternate schema which would be used here is "persistent" for items which are not
    /// deleted each time the application is run. This will be unnecessary once TechTriage is no
    /// longer in early development/testing.
    const SCHEMA_NAME: &str = "main";
    /// The name of the entity in the database.
    ///
    /// It is recommended that all [`DatabaseEntity`] types should have an identical name to the one
    /// they have in the database (with different case conventions, of course), but this is not
    /// assumed in order to be slightly less restrictive.
    const ENTITY_NAME: &str;
    /// The primary key of this entity in the database.
    ///
    /// This is used directly in the SQL for querying the entity, so it should be in the format
    /// expected by SQL. For most entities, this will be a standalone column name, but for junction
    /// tables, it will be multiple column names written as a parenthesized, comma-separated list,
    /// such as `"(column_a, column_b, column_c)"`.
    const PRIMARY_KEY: &str;

    /// Create the entity from a collection of rows.
    // TODO: Take `Into<Vec<Self::Row>>` here
    fn with_rows(rows: Vec<Self::Row>) -> Self;
    /// Convert the entity into a collection of rows.
    fn take_rows(self) -> Vec<Self::Row>;
    /// Borrow the entity's rows.
    fn rows(&self) -> &[Self::Row];

    /// Query (select) a single row from the database using an identifying key.
    ///
    /// If the row exists in the database, it is returned. Otherwise, [`None`] is returned.
    ///
    /// This is the standard version of this method and should not be used as an Axum route handler.
    /// For the handler method, use [`DatabaseEntity::query_one_handler()`].
    async fn query_one(database: &Database, id: impl IdParameter) -> Option<Self::Row> {
        sqlx::query_as(&format!(
            "SELECT * FROM {}.{} WHERE {} = {}",
            Self::SCHEMA_NAME,
            Self::ENTITY_NAME,
            Self::PRIMARY_KEY,
            id.id(),
        ))
        .fetch_one(&database.connection)
        .await
        .ok()
    }

    /// Query (select) a single row from the database using an identifying key.
    ///
    /// If the row exists in the database, it is returned. Otherwise, [`None`] is returned.
    ///
    /// This is the Axum route handler version of this method. For the standard method, which can be
    /// called outside of an Axum context, see [`DatabaseEntity::query_one()`].
    // TODO: Check how this interacts with junction tables
    async fn query_one_handler(
        State(state): State<Arc<ServerState>>,
        Query(id_param): Query<impl IdParameter>,
    ) -> Option<Self::Row> {
        Self::query_one(&state.database, id_param).await
    }

    /// Query (select) all rows for this entity from the database.
    ///
    /// This is the standard version of this method and should not be used as an Axum route handler.
    /// For the handler method, use [`DatabaseEntity::query_all_handler()`].
    async fn query_all(database: &Database) -> Self {
        Self::with_rows(
            sqlx::query_as(&format!(
                "SELECT * FROM {}.{} ORDER BY {}",
                Self::SCHEMA_NAME,
                Self::ENTITY_NAME,
                Self::PRIMARY_KEY
            ))
            .fetch_all(&database.connection)
            .await
            .unwrap(),
        )
    }

    /// Query (select) all rows for this entity from the database.
    ///
    /// This is the Axum route handler version of this method. For the standard method, which can be
    /// called outside of an Axum context, see [`DatabaseEntity::query_all()`].
    async fn query_all_handler(State(state): State<Arc<ServerState>>) -> Self {
        Self::query_all(&state.database).await
    }

    /// Pick a random row from the entity.
    ///
    /// This is used mostly for randomly generating foreign keys, but can be used elsewhere if
    /// needed.
    fn pick_random(&self) -> Self::Row {
        let rows = self.rows();
        rows[thread_rng().gen_range(0..rows.len())].clone()
    }
}

/// A trait that allows table/view row types to interoperate with and be queried from the database.
///
/// This does not implement any insertion methods because "entities" can be views, which are
/// read-only. For inserting items to tables, see the [`SingleInsert`] and [`BulkInsert`] traits.
///
/// This trait mostly exists for use with insertion traits, but also acts as a passthrough to allow
/// items to be queried using the row type instead of the entity type when convenient.
pub trait DatabaseRow: for<'a> sqlx::FromRow<'a, PgRow> + Send + Unpin + Clone {
    /// The entity type which contains a collection of this row type.
    ///
    /// This type and the [`DatabaseEntity::Row`] type are directly interreferential to allow
    /// "upcasting" and "downcasting," mostly for auto-implementations in other traits.
    type Entity: DatabaseEntity<Row = Self>;

    #[allow(dead_code)]
    /// Query (select) a single row from the database using an identifying key.
    ///
    /// If the row exists in the database, it is returned. Otherwise, [`None`] is returned.
    ///
    /// This is the standard version of this method and should not be used as an Axum route handler.
    /// For the handler method, use [`DatabaseRow::query_one_handler()`].
    async fn query_one(database: &Database, id_param: impl IdParameter) -> Option<Self> {
        Self::Entity::query_one(database, id_param).await
    }

    /// Query (select) a single row from the database using an identifying key.
    ///
    /// If the row exists in the database, it is returned. Otherwise, [`None`] is returned.
    ///
    /// This is the Axum route handler version of this method. For the standard method, which can be
    /// called outside of an Axum context, see [`DatabaseRow::query_one()`].
    async fn query_one_handler(
        state: State<Arc<ServerState>>,
        id_param: Query<impl IdParameter>,
    ) -> Option<Self> {
        Self::Entity::query_one_handler(state, id_param).await
    }

    #[allow(dead_code)]
    /// Query (select) all rows for this entity from the database.
    ///
    /// This is the standard version of this method and should not be used as an Axum route handler.
    /// For the handler method, use [`DatabaseRow::query_all_handler()`].
    async fn query_all(database: &Database) -> Self::Entity {
        Self::Entity::query_all(database).await
    }

    #[allow(dead_code)]
    /// Query (select) all rows for this entity from the database.
    ///
    /// This is the Axum route handler version of this method. For the standard method, which can be
    /// called outside of an Axum context, see [`DatabaseRow::query_all()`].
    async fn query_all_handler(state: State<Arc<ServerState>>) -> Self::Entity {
        Self::Entity::query_all_handler(state).await
    }
}

/// A trait that allows a database table to be randomly generated.
///
/// This is used for generating arbitrary quantities of synthetic data to test the application.
trait GenerateTableData: DatabaseEntity<Row: GenerateRowData> {
    /// Randomly generate the database table with a given number of rows.
    ///
    /// Some row types (those with foreign key columns) can only be generated if a set of existing
    /// tables are provided. This means that, when generating multiple database tables, they must be
    /// generated in the correct order such that each will have access to its dependency tables.
    fn generate(
        count: usize,
        dependencies: <Self::Row as GenerateRowData>::Dependencies<'_>,
    ) -> Self {
        let mut rows = Vec::new();
        let mut existing_ids = HashSet::new();
        let mut loading_bar = LoadingBar::new(count);
        for _ in 0..count {
            loading_bar.update();
            rows.push(Self::Row::generate(&rows, &mut existing_ids, dependencies))
        }

        Self::with_rows(rows)
    }
}

/// A trait that allows a database row to be randomly generated.
///
/// This is used for generating arbitrary quantities of synthetic data to test the application.
trait GenerateRowData: Sized {
    /// The primary identifier type for this row.
    ///
    /// Usually this will be an [`i32`] (signed integers are used for database compatibility, even
    /// though negative values are not expected), but if needed it can be any type that can be put
    /// in a [`HashSet`] to ensure that duplicate rows are not generated.
    type Identifier: Copy;
    /// The existing tables which must be provided in order for rows of this type to be generated.
    ///
    /// This should be in the form of a tuple of [`DatabaseEntity`] types.
    ///
    /// It is mandatory to utilize this feature for any row type with one or more foreign key
    /// columns to ensure referential integrity when the rows are inserted into the database.
    type Dependencies<'a>: Copy;

    /// Randomly generate a single row of synthetic data.
    ///
    /// This is usually implemented using a mix of basic RNG and the [`fake`] crate, which can
    /// generate more complex data such as names, phone numbers, email/street addresses, etc. The
    /// implementation must return a row with a unique ID. Any foreign key column must only use IDs
    /// found within its respective dependency table.
    fn generate(
        existing_rows: &[Self],
        existing_ids: &mut HashSet<Self::Identifier>,
        dependencies: Self::Dependencies<'_>,
    ) -> Self;
}

/// A trait that allows a database table to be generated from values known at compile-time.
///
/// This is mostly useful for small tables that have a fixed set of data for whom randomly-generated
/// data would not make sense, such as [`tables::device_categories::DeviceCategoriesDatabaseTable`].
trait GenerateStaticTableData: DatabaseEntity<Row: GenerateStaticRowData> {
    /// The items that are to be inserted into the database table.
    ///
    /// This is a string array because [`GenerateStaticTableData`] is only implemented for simple
    /// tables with ID-string pairs, using the [`GenerateStaticRowData`] trait to convert the
    /// strings to database entries.
    const ITEMS: &[&str];

    /// Generate the table from static data, usually so it can be inserted into the database.
    ///
    /// This is only called `generate` for semantic consistency with the [`GenerateTableData`] trait
    /// which uses actual random data generation.
    fn generate() -> Self {
        let mut existing_ids = HashSet::new();
        let rows = Self::ITEMS
            .iter()
            .map(|item| Self::Row::new(generate_unique_i32(0, &mut existing_ids), *item))
            .collect();

        Self::with_rows(rows)
    }
}

/// A helper trait that allows database rows to be generated using a string.
///
/// This trait should only be implemented for row types with simple ID-string pairs.
trait GenerateStaticRowData {
    /// Turn a string into a database row.
    ///
    /// This method should only be used for [`GenerateStaticTableData::generate`].
    fn new(id: i32, display_name: impl Into<String>) -> Self;
}

/// A trait that allows a single row to be inserted to the database.
///
/// Though generic over [`DatabaseRow`], this trait is only meant to be implemented on database
/// table row types, as items cannot be inserted into a database view. In the future there may be a
/// trait bound to prevent this from happening accidentally.
///
/// For bulk-insertion of rows, see the related [`BulkInsert`] trait.
pub trait SingleInsert: DatabaseRow {
    /// The names of all columns in the database table.
    ///
    /// This was going to be a member of [`DatabaseEntity`] but was placed here because it is needed
    /// for [`SingleInsert::get_query_builder`] to generate the SQL for inserting rows to the
    /// database, as well as determining the [`BulkInsert::CHUNK_SIZE`].
    const COLUMN_NAMES: &[&str];

    /// Get the [`QueryBuilder`] necessary to insert one or more rows of data into the database.
    ///
    /// This is used by both [`SingleInsert`] and [`BulkInsert`] and is meant mostly for
    /// auto-implementations.
    fn get_query_builder<'a>() -> QueryBuilder<'a, Postgres> {
        QueryBuilder::new(&format!(
            "INSERT INTO {}.{} ({}) ",
            Self::Entity::SCHEMA_NAME,
            Self::Entity::ENTITY_NAME,
            Self::COLUMN_NAMES.join(", ")
        ))
    }

    /// Push the row's data into the [`QueryBuilder`] so it can be built and executed against the
    /// database.
    ///
    /// This method is used as a function parameter for [`QueryBuilder::push_values`] and should
    /// only be used within auto-implementations.
    fn push_column_bindings(builder: Separated<Postgres, &str>, row: Self);

    /// Insert the row into the database.
    ///
    /// This should not be used repeatedly for a collection of rows. Inserting multiple rows can be
    /// done much more efficiently using [`BulkInsert::insert_all`], which should be implemented for
    /// any database table type.
    async fn insert(self, database: &Database) {
        let mut query_builder = Self::get_query_builder();
        query_builder.push_values(std::iter::once(self), Self::push_column_bindings);
        database.execute_query_builder(query_builder).await;
    }
}

/// A trait that allows an entire table of rows to be inserted to the database in large batches.
///
/// Bulk-inserting items removes the need for establishing a network connection to the database
/// repeatedly. In initial testing, this proved to be about 20x more efficient than single insertion
/// when working with large tables. Of course, this is mostly used with synthetic data for testing
/// purposes, as it is relatively rare for a significant number of rows to be inserted at once
/// during normal operation.
///
/// Though generic over [`DatabaseEntity`], this trait is only meant to be implemented on database
/// table types, as items cannot be inserted into a database view. In the future there may be a
/// trait bound to prevent this from happening accidentally.
///
/// For single-insertion of rows, see the related [`SingleInsert`] trait.
// TODO: Maybe add a marker `DatabaseTable` trait to prevent this being implemented for view types
pub trait BulkInsert: DatabaseEntity<Row: SingleInsert> {
    /// The amount of rows that can be inserted per batch/chunk.
    ///
    /// The batch limit is determined by the number of columns in a table. This is because a single
    /// SQL statement only supports up to [`u16::MAX`] parameter bindings, and each column takes up
    /// one parameter. Effectively, this means that tables with more columns are split into more
    /// batches, making bulk insertion take longer.
    const CHUNK_SIZE: usize = SQL_PARAMETER_BIND_LIMIT / Self::Row::COLUMN_NAMES.len();

    /// Convert a table of rows into a series of batches to be inserted to the database.
    ///
    /// This method should only be used within auto-implementations.
    fn into_chunks(self) -> impl Iterator<Item = Vec<Self::Row>> {
        let mut iter = self.take_rows().into_iter();
        // TODO: Annotate this code or something, I have very little idea what it does
        // * This was done because `itertools::IntoChunks` was causing issues with the axum handlers
        std::iter::from_fn(move || Some(iter.by_ref().take(Self::CHUNK_SIZE).collect()))
            .take_while(|v: &Vec<_>| !v.is_empty())
    }

    /// Insert the entire table into the database in a series of batches (or "chunks").
    ///
    /// This can insert tables of arbitrary size, but each batch is limited in size by number of
    /// parameters (table column count * row count).
    async fn insert_all(self, database: &Database) {
        for chunk in self.into_chunks() {
            let mut query_builder = Self::Row::get_query_builder();
            query_builder.push_values(chunk, Self::Row::push_column_bindings);
            database.execute_query_builder(query_builder).await;
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
        let start_time = Instant::now();

        let device_categories = DeviceCategoriesDatabaseTable::generate();
        device_categories.clone().insert_all(self).await;
        let part_categories = PartCategoriesDatabaseTable::generate();
        part_categories.clone().insert_all(self).await;
        let service_types = ServiceTypesDatabaseTable::generate();
        service_types.clone().insert_all(self).await;

        println!("Generating {VENDORS_COUNT} vendors");
        let vendors = VendorsDatabaseTable::generate(VENDORS_COUNT, ());
        vendors.clone().insert_all(self).await;

        println!("Generating {DEVICE_MANUFACTURERS_COUNT} device manufacturers");
        let device_manufacturers =
            DeviceManufacturersDatabaseTable::generate(DEVICE_MANUFACTURERS_COUNT, ());
        device_manufacturers.clone().insert_all(self).await;

        println!("Generating {PART_MANUFACTURERS_COUNT} part manufacturers");
        let part_manufacturers =
            PartManufacturersDatabaseTable::generate(PART_MANUFACTURERS_COUNT, ());
        part_manufacturers.clone().insert_all(self).await;

        println!("Generating {DEVICE_MODELS_COUNT} device models");
        let device_models = DeviceModelsDatabaseTable::generate(
            DEVICE_MODELS_COUNT,
            (&device_manufacturers, &device_categories),
        );
        device_models.clone().insert_all(self).await;

        println!("Generating {PARTS_COUNT} parts");
        let parts = PartsDatabaseTable::generate(
            PARTS_COUNT,
            (&vendors, &part_manufacturers, &part_categories),
        );
        parts.clone().insert_all(self).await;

        println!("Generating {PRODUCTS_COUNT} products");
        let products = ProductsDatabaseTable::generate(PRODUCTS_COUNT, ());
        products.clone().insert_all(self).await;

        println!("Generating {PRODUCT_PRICES_COUNT} product_prices");
        let product_prices = ProductPricesDatabaseTable::generate(PRODUCT_PRICES_COUNT, &products);
        product_prices.clone().insert_all(self).await;

        println!("Generating {SERVICES_COUNT} services");
        let services =
            ServicesDatabaseTable::generate(SERVICES_COUNT, (&service_types, &device_models));
        services.clone().insert_all(self).await;

        println!("Generating {SERVICE_PRICES_COUNT} service_prices");
        let service_prices = ServicePricesDatabaseTable::generate(SERVICE_PRICES_COUNT, &services);
        service_prices.clone().insert_all(self).await;

        println!("Generating {CUSTOMERS_COUNT} customers");
        let customers = CustomersDatabaseTable::generate(CUSTOMERS_COUNT, ());
        customers.clone().insert_all(self).await;

        println!("Generating {DEVICES_COUNT} devices");
        let devices = DevicesDatabaseTable::generate(DEVICES_COUNT, (&device_models, &customers));
        devices.clone().insert_all(self).await;

        // * Items must be fetched from the database as they are generated by triggers when
        // * inserting products and services and not separately generated.
        let items = ItemsDatabaseTable::query_all(self).await;

        println!("Generating {INVOICES_COUNT} invoices");
        let invoices = InvoicesDatabaseTable::generate(INVOICES_COUNT, ());
        invoices.clone().insert_all(self).await;

        println!("Generating {INVOICE_ITEMS_COUNT} invoice items");
        let invoice_items =
            InvoiceItemsDatabaseTable::generate(INVOICE_ITEMS_COUNT, (&invoices, &items));
        invoice_items.clone().insert_all(self).await;

        println!("Generating {INVOICE_PAYMENTS_COUNT} invoice payments");
        let invoice_payments = InvoicePaymentsDatabaseTable::generate(
            INVOICE_PAYMENTS_COUNT,
            (
                &invoices,
                &invoice_items,
                &items,
                &product_prices,
                &service_prices,
            ),
        );
        invoice_payments.insert_all(self).await;

        println!("Generating {TICKETS_COUNT} tickets");
        let tickets = TicketsDatabaseTable::generate(TICKETS_COUNT, (&customers, &invoices));
        tickets.clone().insert_all(self).await;

        println!("Generating {COMPATIBLE_PARTS_COUNT} compatible parts");
        let compatible_parts = CompatiblePartsDatabaseJunctionTable::generate(
            COMPATIBLE_PARTS_COUNT,
            (&device_models, &parts),
        );
        compatible_parts.insert_all(self).await;

        println!("Generating {TICKET_DEVICES_COUNT} ticket devices");
        let ticket_devices = TicketDevicesDatabaseJunctionTable::generate(
            TICKET_DEVICES_COUNT,
            (&tickets, &devices, &services),
        );
        ticket_devices.clone().insert_all(self).await;

        println!("Generating {BUNDLED_PARTS_COUNT} bundled parts");
        let bundled_parts = BundledPartsDatabaseJunctionTable::generate(
            BUNDLED_PARTS_COUNT,
            (&ticket_devices, &parts),
        );
        bundled_parts.insert_all(self).await;

        println!(
            "Generated and inserted {} items in {}ms",
            (VENDORS_COUNT
                + DEVICE_MANUFACTURERS_COUNT
                + PART_MANUFACTURERS_COUNT
                + DEVICE_MODELS_COUNT
                + PARTS_COUNT
                + PRODUCTS_COUNT
                + PRODUCT_PRICES_COUNT
                + SERVICES_COUNT
                + SERVICE_PRICES_COUNT
                + CUSTOMERS_COUNT
                + DEVICES_COUNT
                + INVOICES_COUNT
                + INVOICE_ITEMS_COUNT
                + INVOICE_PAYMENTS_COUNT
                + TICKETS_COUNT
                + COMPATIBLE_PARTS_COUNT
                + TICKET_DEVICES_COUNT
                + BUNDLED_PARTS_COUNT),
            start_time.elapsed().as_millis()
        );
    }

    async fn execute_query_builder<'a>(&self, mut query_builder: QueryBuilder<'a, Postgres>) {
        query_builder
            .build()
            .execute(&self.connection)
            .await
            .unwrap();
    }
}
