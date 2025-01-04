mod loading_bar;
pub mod shared_models;
pub mod tables;
pub mod traits;
pub mod views;

use std::time::Instant;

use sqlx::query_builder::QueryBuilder;
use sqlx::{raw_sql, PgPool, Postgres};

use tables::bundled_parts::BundledPartsJunctionTable;
use tables::compatible_parts::CompatiblePartsJunctionTable;
use tables::customers::CustomersTable;
use tables::device_categories::DeviceCategoriesTable;
use tables::device_manufacturers::DeviceManufacturersTable;
use tables::device_models::DeviceModelsTable;
use tables::devices::DevicesTable;
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
use traits::generate::{GenerateStaticRecord, GenerateStaticTable, GenerateTable};
use traits::read::ReadRelation;
use traits::write::BulkInsert;

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
