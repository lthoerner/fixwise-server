mod loading_bar;
pub mod shared_models;
pub mod tables;
pub mod views;

use std::collections::HashSet;
use std::sync::Arc;
use std::time::Instant;

use axum::extract::{Query, State};

use axum::extract::Json;
use rand::{thread_rng, Rng};
use sqlx::postgres::PgRow;
use sqlx::query_builder::{QueryBuilder, Separated};
use sqlx::{raw_sql, PgPool, Postgres};

use crate::api::IdParameter;
use crate::ServerState;
use loading_bar::LoadingBar;
use tables::bundled_parts::BundledPartsJunctionTable;
use tables::compatible_parts::CompatiblePartsJunctionTable;
use tables::customers::CustomersTable;
use tables::device_categories::DeviceCategoriesTable;
use tables::device_manufacturers::DeviceManufacturersTable;
use tables::device_models::DeviceModelsTable;
use tables::devices::DevicesTable;
use tables::generators::*;
use tables::invoice_items::InvoiceItemsTable;
use tables::invoice_payments::InvoicePaymentsTable;
use tables::invoices::InvoicesTable;
use tables::items::ItemsTable;
use tables::part_categories::PartCategoriesTable;
use tables::part_manufacturers::PartManufacturersTable;
use tables::parts::PartsTable;
use tables::product_prices::ProductPricesTable;
use tables::products::ProductsTable;
use tables::service_prices::ServicePricesTable;
use tables::service_types::ServiceTypesTable;
use tables::services::ServicesTable;
use tables::ticket_devices::TicketDevicesJunctionTable;
use tables::tickets::TicketsTable;
use tables::vendors::VendorsTable;

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
const INVOICE_ITEMS_COUNT: usize = 12345;
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
/// This does not implement any insertion or deletion methods because "relations" can be views,
/// which are read-only. For inserting items to tables, see the [`SingleInsert`] and [`BulkInsert`]
/// traits. For deleting items from tables, see the [`Table`] trait.
///
/// This trait does not do a lot on its own but it, along with [`Record`], provides the
/// functionality which allows almost all of the other database traits to be auto-implemented or
/// conveniently derived.
pub trait Relation: Sized {
    /// The record type which this relation contains a collection of.
    ///
    /// This type and the [`Record::Relation`] type are directly interreferential to allow
    /// "upcasting" and "downcasting," mostly for auto-implementations in other traits.
    type Record: Record<Relation = Self>;

    /// The name of the schema in which this relation exists in the database.
    ///
    /// This defaults to "main" but can be changed in case a relation lives in a different schema.
    /// The main alternate schema which would be used here is "persistent" for items which are not
    /// deleted each time the application is run. This will be unnecessary once Fixwise is no longer
    /// in early development/testing.
    const SCHEMA_NAME: &str = "main";
    /// The name of the relation in the database.
    ///
    /// It is recommended that all [`Relation`] types should have an identical name to the one they
    /// have in the database (with different case conventions, of course), but this is not assumed
    /// in order to be slightly less restrictive.
    const RELATION_NAME: &str;
    /// The primary column of this relation in the database.
    ///
    /// This is used directly in the SQL for querying the relation, so it should be in the format
    /// expected by SQL. For most relations, this will be a standalone column name, but for junction
    /// tables, it will be multiple column names written as a parenthesized, comma-separated list,
    /// such as `"(column_a, column_b, column_c)"`.
    const PRIMARY_KEY: &str;

    /// Create the relation from a collection of records.
    // TODO: Take `Into<Vec<Self::Record>>` here
    fn with_records(records: Vec<Self::Record>) -> Self;
    /// Convert the relation into a collection of records.
    fn take_records(self) -> Vec<Self::Record>;
    /// Borrow the relation's records.
    fn records(&self) -> &[Self::Record];

    /// Query (select) a single record from the database using an identifying key.
    ///
    /// If the record exists in the database, it is returned. Otherwise, [`None`] is returned.
    ///
    /// This is the standard version of this method and should not be used as an Axum route handler.
    /// For the handler method, use [`Relation::query_one_handler()`].
    async fn query_one<I: IdParameter>(database: &Database, id: I) -> Option<Self::Record> {
        sqlx::query_as(&format!(
            "SELECT * FROM {}.{} WHERE {} = #1",
            Self::SCHEMA_NAME,
            Self::RELATION_NAME,
            Self::PRIMARY_KEY,
        ))
        .bind(id.id() as i32)
        .fetch_one(&database.connection)
        .await
        .ok()
    }

    /// Query (select) a single record from the database using an identifying key.
    ///
    /// If the record exists in the database, it is returned. Otherwise, [`None`] is returned.
    ///
    /// This is the Axum route handler version of this method. For the standard method, which can be
    /// called outside of an Axum context, see [`Relation::query_one()`].
    // TODO: Check how this interacts with junction tables
    async fn query_one_handler<I: IdParameter>(
        State(state): State<Arc<ServerState>>,
        Query(id_param): Query<I>,
    ) -> Option<Self::Record> {
        Self::query_one(&state.database, id_param).await
    }

    /// Query (select) all records for this relation from the database.
    ///
    /// This is the standard version of this method and should not be used as an Axum route handler.
    /// For the handler method, use [`Relation::query_all_handler()`].
    async fn query_all(database: &Database) -> Self {
        Self::with_records(
            sqlx::query_as(&format!(
                "SELECT * FROM {}.{} ORDER BY {}",
                Self::SCHEMA_NAME,
                Self::RELATION_NAME,
                Self::PRIMARY_KEY,
            ))
            .fetch_all(&database.connection)
            .await
            .unwrap(),
        )
    }

    /// Query (select) all records for this relation from the database.
    ///
    /// This is the Axum route handler version of this method. For the standard method, which can be
    /// called outside of an Axum context, see [`Relation::query_all()`].
    async fn query_all_handler(State(state): State<Arc<ServerState>>) -> Self {
        Self::query_all(&state.database).await
    }

    /// Pick a random record from the relation.
    ///
    /// This is used mostly for randomly generating foreign keys, but can be used elsewhere if
    /// needed.
    fn pick_random(&self) -> Self::Record {
        let records = self.records();
        records[thread_rng().gen_range(0..records.len())].clone()
    }
}

pub trait Table: Relation {
    /// The name given to foreign keys pointing to this relation from other relations.
    ///
    /// This key name must be the same for every dependent table. Usually it will just be the
    /// singular version of the table name ("tickets" becomes "ticket", etc.).
    const FOREIGN_KEY_NAME: &str;
    /// The tables which contain foreign keys pointing to this relation.
    ///
    /// This is used to ensure referential integrity when deleting records from the database. The
    /// tables must be defined in an order that ensures no foreign key constraint still exists when
    /// deleting any record. This would mostly be relevant when one dependent table has a foreign
    /// key that references another dependent table.
    const DEPENDENT_TABLES: &[&str] = &[];

    /// Delete a single record from the database using an identifying key.
    ///
    /// If the record is successfully deleted from the database, this method returns `true`. If an
    /// error occurs, such as if the record does not exist in the database, `false` is returned.
    ///
    /// This is the standard version of this method and should not be used as an Axum route handler.
    /// For the handler method, use [`Table::delete_one_handler()`].
    // TODO: Return a more useful value for error handling
    async fn delete_one<I: IdParameter>(database: &Database, id: I) -> bool {
        for dependent_table in Self::DEPENDENT_TABLES {
            if sqlx::query(&format!(
                "DELETE FROM {dependent_table} WHERE {} = $1",
                Self::FOREIGN_KEY_NAME
            ))
            .bind(id.id() as i32)
            .execute(&database.connection)
            .await
            .is_err()
            {
                return false;
            }
        }

        sqlx::query(&format!(
            "DELETE FROM {}.{} WHERE {} = $1",
            Self::SCHEMA_NAME,
            Self::RELATION_NAME,
            Self::PRIMARY_KEY,
        ))
        .bind(id.id() as i32)
        .execute(&database.connection)
        .await
        .is_ok()
    }

    /// Delete a single record from the database using an identifying key.
    ///
    /// If the record is successfully deleted from the database, this method returns `true`. If an
    /// error occurs, such as if the record does not exist in the database, `false` is returned.
    ///
    /// This is the Axum route handler version of this method. For the standard method, which can be
    /// called outside of an Axum context, see [`Table::delete_one()`].
    async fn delete_one_handler<I: IdParameter>(
        State(state): State<Arc<ServerState>>,
        Query(id_param): Query<I>,
    ) -> Json<bool> {
        Json(Self::delete_one(&state.database, id_param).await)
    }

    /// Delete all records for this relation from the database.
    ///
    /// If the records are successfully deleted from the database, this method returns `true`. If an
    /// error occurs, `false` is returned.
    ///
    /// This is the standard version of this method and should not be used as an Axum route handler.
    /// For the handler method, use [`Table::delete_all_handler()`].
    async fn delete_all(database: &Database) -> bool {
        for dependent_table in Self::DEPENDENT_TABLES {
            if sqlx::query(&format!("DELETE FROM {dependent_table}"))
                .execute(&database.connection)
                .await
                .is_err()
            {
                return false;
            }
        }

        sqlx::query(&format!(
            "DELETE FROM {}.{}",
            Self::SCHEMA_NAME,
            Self::RELATION_NAME,
        ))
        .execute(&database.connection)
        .await
        .is_ok()
    }

    /// Delete all records for this relation from the database.
    ///
    /// If the records are successfully deleted from the database, this method returns `true`. If an
    /// error occurs, `false` is returned.
    ///
    /// This is the Axum route handler version of this method. For the standard method, which can be
    /// called outside of an Axum context, see [`Table::delete_all()`].
    async fn delete_all_handler(State(state): State<Arc<ServerState>>) -> bool {
        Self::delete_all(&state.database).await
    }
}

/// A trait that allows table/view record types to interoperate with and be queried from the
/// database.
///
/// This does not implement any insertion methods because "relations" can be views, which are
/// read-only. For inserting items to tables, see the [`SingleInsert`] and [`BulkInsert`] traits.
///
/// This trait mostly exists for use with insertion traits, but also acts as a passthrough to allow
/// items to be queried using the record type instead of the relation type when convenient.
pub trait Record: for<'a> sqlx::FromRow<'a, PgRow> + Send + Unpin + Clone {
    /// The relation type which contains a collection of this record type.
    ///
    /// This type and the [`Relation::Record`] type are directly interreferential to allow
    /// "upcasting" and "downcasting," mostly for auto-implementations in other traits.
    type Relation: Relation<Record = Self>;

    #[allow(dead_code)]
    /// Query (select) a single record from the database using an identifying key.
    ///
    /// If the record exists in the database, it is returned. Otherwise, [`None`] is returned.
    ///
    /// This is the standard version of this method and should not be used as an Axum route handler.
    /// For the handler method, use [`Record::query_one_handler()`].
    async fn query_one<I: IdParameter>(database: &Database, id_param: I) -> Option<Self> {
        Self::Relation::query_one(database, id_param).await
    }

    /// Query (select) a single record from the database using an identifying key.
    ///
    /// If the record exists in the database, it is returned. Otherwise, [`None`] is returned.
    ///
    /// This is the Axum route handler version of this method. For the standard method, which can be
    /// called outside of an Axum context, see [`Record::query_one()`].
    async fn query_one_handler<I: IdParameter>(
        state: State<Arc<ServerState>>,
        id_param: Query<I>,
    ) -> Option<Self> {
        Self::Relation::query_one_handler(state, id_param).await
    }

    #[allow(dead_code)]
    /// Query (select) all records for this relation from the database.
    ///
    /// This is the standard version of this method and should not be used as an Axum route handler.
    /// For the handler method, use [`Record::query_all_handler()`].
    async fn query_all(database: &Database) -> Self::Relation {
        Self::Relation::query_all(database).await
    }

    #[allow(dead_code)]
    /// Query (select) all records for this relation from the database.
    ///
    /// This is the Axum route handler version of this method. For the standard method, which can be
    /// called outside of an Axum context, see [`Record::query_all()`].
    async fn query_all_handler(state: State<Arc<ServerState>>) -> Self::Relation {
        Self::Relation::query_all_handler(state).await
    }
}

pub trait TableRecord: Record<Relation: Table> {
    #[allow(dead_code)]
    /// Delete a single record from the database using an identifying key.
    ///
    /// If the record is successfully deleted from the database, this method returns `true`. If an
    /// error occurs, such as if the record does not exist in the database, `false` is returned.
    ///
    /// This is the standard version of this method and should not be used as an Axum route handler.
    /// For the handler method, use [`TableRecord::delete_one_handler()`].
    async fn delete_one<I: IdParameter>(database: &Database, id: I) -> bool {
        Self::Relation::delete_one(database, id).await
    }

    #[allow(dead_code)]
    /// Delete a single record from the database using an identifying key.
    ///
    /// If the record is successfully deleted from the database, this method returns `true`. If an
    /// error occurs, such as if the record does not exist in the database, `false` is returned.
    ///
    /// This is the Axum route handler version of this method. For the standard method, which can be
    /// called outside of an Axum context, see [`TableRecord::delete_one()`].
    async fn delete_one_handler<I: IdParameter>(
        state: State<Arc<ServerState>>,
        id_param: Query<I>,
    ) -> Json<bool> {
        Self::Relation::delete_one_handler(state, id_param).await
    }

    #[allow(dead_code)]
    /// Delete all records for this relation from the database.
    ///
    /// If the records are successfully deleted from the database, this method returns `true`. If an
    /// error occurs, `false` is returned.
    ///
    /// This is the standard version of this method and should not be used as an Axum route handler.
    /// For the handler method, use [`TableRecord::delete_all_handler()`].
    async fn delete_all(database: &Database) -> bool {
        Self::Relation::delete_all(database).await
    }

    #[allow(dead_code)]
    /// Delete all records for this relation from the database.
    ///
    /// If the records are successfully deleted from the database, this method returns `true`. If an
    /// error occurs, `false` is returned.
    ///
    /// This is the Axum route handler version of this method. For the standard method, which can be
    /// called outside of an Axum context, see [`TableRecord::delete_all()`].
    async fn delete_all_handler(state: State<Arc<ServerState>>) -> bool {
        Self::Relation::delete_all_handler(state).await
    }
}

/// A trait that allows a database table to be randomly generated.
///
/// This is used for generating arbitrary quantities of synthetic data to test the application.
trait GenerateTableData: Relation<Record: GenerateRecord> {
    /// Randomly generate the database table with a given number of records.
    ///
    /// Some record types (those with foreign key columns) can only be generated if a set of
    /// existing tables are provided. This means that, when generating multiple database tables,
    /// they must be generated in the correct order such that each will have access to its
    /// dependency tables.
    fn generate(
        count: usize,
        dependencies: <Self::Record as GenerateRecord>::Dependencies<'_>,
    ) -> Self {
        let mut records = Vec::new();
        let mut existing_ids = HashSet::new();
        let mut loading_bar = LoadingBar::new(count);
        for _ in 0..count {
            loading_bar.update();
            records.push(Self::Record::generate(
                &records,
                &mut existing_ids,
                dependencies,
            ))
        }

        Self::with_records(records)
    }
}

/// A trait that allows a database record to be randomly generated.
///
/// This is used for generating arbitrary quantities of synthetic data to test the application.
trait GenerateRecord: Sized {
    /// The primary identifier type for this record.
    ///
    /// Usually this will be an [`i32`] (signed integers are used for database compatibility, even
    /// though negative values are not expected), but if needed it can be any type that can be put
    /// in a [`HashSet`] to ensure that duplicate records are not generated.
    type Identifier: Copy;
    /// The existing tables which must be provided in order for records of this type to be
    /// generated.
    ///
    /// This should be in the form of a tuple of [`Relation`] types.
    ///
    /// It is mandatory to utilize this feature for any record type with one or more foreign key
    /// columns to ensure referential integrity when the records are inserted into the database.
    type Dependencies<'a>: Copy;

    /// Randomly generate a single record of synthetic data.
    ///
    /// This is usually implemented using a mix of basic RNG and the [`fake`] crate, which can
    /// generate more complex data such as names, phone numbers, email/street addresses, etc. The
    /// implementation must return a record with a unique ID. Any foreign key column must only use
    /// IDs found within its respective dependency table.
    fn generate(
        existing_records: &[Self],
        existing_ids: &mut HashSet<Self::Identifier>,
        dependencies: Self::Dependencies<'_>,
    ) -> Self;
}

/// A trait that allows a database table to be generated from values known at compile-time.
///
/// This is mostly useful for small tables that have a fixed set of data for whom randomly-generated
/// data would not make sense, such as [`tables::device_categories::DeviceCategoriesTable`].
trait GenerateStaticRelation: Relation<Record: GenerateStaticRecord> {
    /// The items that are to be inserted into the database table.
    ///
    /// This is a string array because [`GenerateStaticRelation`] is only implemented for simple
    /// tables with ID-string pairs, using the [`GenerateStaticRecord`] trait to convert the strings
    /// to database entries.
    const ITEMS: &[&str];

    /// Generate the table from static data, usually so it can be inserted into the database.
    ///
    /// This is only called `generate` for semantic consistency with the [`GenerateTableData`] trait
    /// which uses actual random data generation.
    fn generate() -> Self {
        let mut existing_ids = HashSet::new();
        let records = Self::ITEMS
            .iter()
            .map(|item| Self::Record::new(generate_unique_i32(0, &mut existing_ids), *item))
            .collect();

        Self::with_records(records)
    }
}

/// A helper trait that allows database records to be generated using a string.
///
/// This trait should only be implemented for record types with simple ID-string pairs.
trait GenerateStaticRecord {
    /// Turn a string into a database record.
    ///
    /// This method should only be used for [`GenerateStaticRelation::generate`].
    fn new(id: i32, display_name: impl Into<String>) -> Self;
}

/// A trait that allows a single record to be inserted to the database.
///
/// Though generic over [`Record`], this trait is only meant to be implemented on database table
/// record types, as items cannot be inserted into a database view. In the future there may be a
/// trait bound to prevent this from happening accidentally.
///
/// For bulk-insertion of records, see the related [`BulkInsert`] trait.
pub trait SingleInsert: TableRecord {
    /// The names of all columns in the database table.
    ///
    /// This was going to be a member of [`Table`] but was placed here because it is needed for
    /// [`SingleInsert::get_query_builder`] to generate the SQL for inserting records to the
    /// database, as well as determining the [`BulkInsert::CHUNK_SIZE`].
    const COLUMN_NAMES: &[&str];

    /// Get the [`QueryBuilder`] necessary to insert one or more records of data into the database.
    ///
    /// This is used by both [`SingleInsert`] and [`BulkInsert`] and is meant mostly for
    /// auto-implementations.
    fn get_query_builder<'a>() -> QueryBuilder<'a, Postgres> {
        QueryBuilder::new(&format!(
            "INSERT INTO {}.{} ({}) ",
            Self::Relation::SCHEMA_NAME,
            Self::Relation::RELATION_NAME,
            Self::COLUMN_NAMES.join(", ")
        ))
    }

    /// Push the record's data into the [`QueryBuilder`] so it can be built and executed against the
    /// database.
    ///
    /// This method is used as a function parameter for [`QueryBuilder::push_values`] and should
    /// only be used within auto-implementations.
    fn push_column_bindings(builder: Separated<Postgres, &str>, record: Self);

    /// Insert the record into the database.
    ///
    /// This should not be used repeatedly for a collection of records. Inserting multiple records
    /// can be done much more efficiently using [`BulkInsert::insert_all`], which should be
    /// implemented for any database table type.
    async fn insert(self, database: &Database) {
        let mut query_builder = Self::get_query_builder();
        query_builder.push_values(std::iter::once(self), Self::push_column_bindings);
        database.execute_query_builder(query_builder).await;
    }
}

/// A trait that allows an entire table of records to be inserted to the database in large batches.
///
/// Bulk-inserting items removes the need for establishing a network connection to the database
/// repeatedly. In initial testing, this proved to be about 20x more efficient than single insertion
/// when working with large tables. Of course, this is mostly used with synthetic data for testing
/// purposes, as it is relatively rare for a significant number of records to be inserted at once
/// during normal operation.
///
/// For single-insertion of records, see the related [`SingleInsert`] trait.
pub trait BulkInsert: Table<Record: SingleInsert> {
    /// The amount of records that can be inserted per batch/chunk.
    ///
    /// The batch limit is determined by the number of columns in a table. This is because a single
    /// SQL statement only supports up to [`u16::MAX`] parameter bindings, and each column takes up
    /// one parameter. Effectively, this means that tables with more columns are split into more
    /// batches, making bulk insertion take longer.
    const CHUNK_SIZE: usize = SQL_PARAMETER_BIND_LIMIT / Self::Record::COLUMN_NAMES.len();

    /// Convert a table of records into a series of batches to be inserted to the database.
    ///
    /// This method should only be used within auto-implementations.
    fn into_chunks(self) -> impl Iterator<Item = Vec<Self::Record>> {
        let mut iter = self.take_records().into_iter();
        // TODO: Annotate this code or something, I have very little idea what it does
        // * This was done because `itertools::IntoChunks` was causing issues with the axum handlers
        std::iter::from_fn(move || Some(iter.by_ref().take(Self::CHUNK_SIZE).collect()))
            .take_while(|v: &Vec<_>| !v.is_empty())
    }

    /// Insert the entire table into the database in a series of batches (or "chunks").
    ///
    /// This can insert tables of arbitrary size, but each batch is limited in size by number of
    /// parameters (table column count * record count).
    async fn insert_all(self, database: &Database) {
        for chunk in self.into_chunks() {
            let mut query_builder = Self::Record::get_query_builder();
            query_builder.push_values(chunk, Self::Record::push_column_bindings);
            database.execute_query_builder(query_builder).await;
        }
    }
}

impl Database {
    const CONFIG_SCRIPT: &str = include_str!("../../database/config.pgsql");

    pub async fn connect_and_configure() -> Self {
        let database = Self::connect().await;
        database.configure().await;

        database
    }

    async fn connect() -> Self {
        Self {
            connection: PgPool::connect("postgresql://fixwise:fixwise@localhost:5432")
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

        let device_categories = DeviceCategoriesTable::generate();
        device_categories.clone().insert_all(self).await;
        let part_categories = PartCategoriesTable::generate();
        part_categories.clone().insert_all(self).await;
        let service_types = ServiceTypesTable::generate();
        service_types.clone().insert_all(self).await;

        eprintln!("Generating {VENDORS_COUNT} vendors");
        let vendors = VendorsTable::generate(VENDORS_COUNT, ());
        vendors.clone().insert_all(self).await;

        eprintln!("Generating {DEVICE_MANUFACTURERS_COUNT} device manufacturers");
        let device_manufacturers =
            DeviceManufacturersTable::generate(DEVICE_MANUFACTURERS_COUNT, ());
        device_manufacturers.clone().insert_all(self).await;

        eprintln!("Generating {PART_MANUFACTURERS_COUNT} part manufacturers");
        let part_manufacturers = PartManufacturersTable::generate(PART_MANUFACTURERS_COUNT, ());
        part_manufacturers.clone().insert_all(self).await;

        eprintln!("Generating {DEVICE_MODELS_COUNT} device models");
        let device_models = DeviceModelsTable::generate(
            DEVICE_MODELS_COUNT,
            (&device_manufacturers, &device_categories),
        );
        device_models.clone().insert_all(self).await;

        eprintln!("Generating {PARTS_COUNT} parts");
        let parts = PartsTable::generate(
            PARTS_COUNT,
            (&vendors, &part_manufacturers, &part_categories),
        );
        parts.clone().insert_all(self).await;

        eprintln!("Generating {PRODUCTS_COUNT} products");
        let products = ProductsTable::generate(PRODUCTS_COUNT, ());
        products.clone().insert_all(self).await;

        eprintln!("Generating {PRODUCT_PRICES_COUNT} product_prices");
        let product_prices = ProductPricesTable::generate(PRODUCT_PRICES_COUNT, &products);
        product_prices.clone().insert_all(self).await;

        eprintln!("Generating {SERVICES_COUNT} services");
        let services = ServicesTable::generate(SERVICES_COUNT, (&service_types, &device_models));
        services.clone().insert_all(self).await;

        eprintln!("Generating {SERVICE_PRICES_COUNT} service_prices");
        let service_prices = ServicePricesTable::generate(SERVICE_PRICES_COUNT, &services);
        service_prices.clone().insert_all(self).await;

        eprintln!("Generating {CUSTOMERS_COUNT} customers");
        let customers = CustomersTable::generate(CUSTOMERS_COUNT, ());
        customers.clone().insert_all(self).await;

        eprintln!("Generating {DEVICES_COUNT} devices");
        let devices = DevicesTable::generate(DEVICES_COUNT, (&device_models, &customers));
        devices.clone().insert_all(self).await;

        // * Items must be fetched from the database as they are generated by triggers when
        // * inserting products and services and not separately generated.
        let items = ItemsTable::query_all(self).await;

        println!("Generating {INVOICES_COUNT} invoices");
        let invoices = InvoicesTable::generate(INVOICES_COUNT, ());
        invoices.clone().insert_all(self).await;

        println!("Generating {INVOICE_ITEMS_COUNT} invoice items");
        let invoice_items = InvoiceItemsTable::generate(INVOICE_ITEMS_COUNT, (&invoices, &items));
        invoice_items.clone().insert_all(self).await;

        println!("Generating {INVOICE_PAYMENTS_COUNT} invoice payments");
        let invoice_payments = InvoicePaymentsTable::generate(
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
        let tickets = TicketsTable::generate(TICKETS_COUNT, (&customers, &invoices));
        tickets.clone().insert_all(self).await;

        println!("Generating {COMPATIBLE_PARTS_COUNT} compatible parts");
        let compatible_parts = CompatiblePartsJunctionTable::generate(
            COMPATIBLE_PARTS_COUNT,
            (&device_models, &parts),
        );
        compatible_parts.insert_all(self).await;

        println!("Generating {TICKET_DEVICES_COUNT} ticket devices");
        let ticket_devices = TicketDevicesJunctionTable::generate(
            TICKET_DEVICES_COUNT,
            (&tickets, &devices, &services),
        );
        ticket_devices.clone().insert_all(self).await;

        println!("Generating {BUNDLED_PARTS_COUNT} bundled parts");
        let bundled_parts =
            BundledPartsJunctionTable::generate(BUNDLED_PARTS_COUNT, (&ticket_devices, &parts));
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
