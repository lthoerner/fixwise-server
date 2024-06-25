mod api;
mod database;

use std::sync::Arc;
use std::time::Instant;

use axum::routing::get;
use axum::Router;
use http::Method;
use tokio::net::TcpListener;
use tokio::signal;
use tower_http::cors::{Any, CorsLayer};

use api::views::customers::CustomersApiView;
use api::views::tickets::TicketsApiView;
use api::ServeJson;
use database::tables::bundled_parts::BundledPartsDatabaseJunctionTable;
use database::tables::compatible_parts::CompatiblePartsDatabaseJunctionTable;
use database::tables::customers::CustomersDatabaseTable;
use database::tables::device_categories::DeviceCategoriesDatabaseTable;
use database::tables::device_manufacturers::DeviceManufacturersDatabaseTable;
use database::tables::device_models::DeviceModelsDatabaseTable;
use database::tables::devices::DevicesDatabaseTable;
use database::tables::part_categories::PartCategoriesDatabaseTable;
use database::tables::part_manufacturers::PartManufacturersDatabaseTable;
use database::tables::parts::PartsDatabaseTable;
use database::tables::ticket_devices::TicketDevicesDatabaseJunctionTable;
use database::tables::tickets::TicketsDatabaseTable;
use database::tables::vendors::VendorsDatabaseTable;
use database::Database;

#[derive(Clone)]
struct ServerState {
    database: Database,
}

#[tokio::main]
async fn main() {
    println!("Connecting to database...");
    let server_state = Arc::new(ServerState {
        database: Database::connect_and_configure().await,
    });

    let signal_handler_server_state = server_state.clone();
    tokio::spawn(async move {
        signal::ctrl_c().await.unwrap();
        println!();
        println!("Server shutting down...");
        signal_handler_server_state
            .database
            .connection
            .close()
            .await;
        println!("Database connection closed.");
        std::process::exit(0);
    });

    let num_vendors = 123;
    let num_device_manufacturers = 123;
    let num_part_manufacturers = 123;
    let num_device_models = 123;
    let num_devices = 1234;
    let num_parts = 1234;
    let num_customers = 1234;
    let num_tickets = 1234;
    let num_compatible_parts = 1234;
    let num_ticket_devices = 1234;
    let num_bundled_parts = 1234;

    let device_categories = DeviceCategoriesDatabaseTable::generate();
    let part_categories = PartCategoriesDatabaseTable::generate();
    println!("Generating {num_customers} customers");
    let customers = CustomersDatabaseTable::generate(num_customers);
    println!("Generating {num_vendors} vendors");
    let vendors = VendorsDatabaseTable::generate(num_vendors);
    println!("Generating {num_device_manufacturers} device manufacturers");
    let device_manufacturers = DeviceManufacturersDatabaseTable::generate(num_device_manufacturers);
    println!("Generating {num_part_manufacturers} part manufacturers");
    let part_manufacturers = PartManufacturersDatabaseTable::generate(num_part_manufacturers);
    println!("Generating {num_device_models} device models");
    let device_models = DeviceModelsDatabaseTable::generate(
        num_device_models,
        &device_manufacturers,
        &device_categories,
    );
    println!("Generating {num_devices} devices");
    let devices = DevicesDatabaseTable::generate(num_devices, &device_models, &customers);
    println!("Generating {num_parts} parts");
    let parts =
        PartsDatabaseTable::generate(num_parts, &vendors, &part_manufacturers, &part_categories);
    println!("Generating {num_tickets} tickets");
    let tickets = TicketsDatabaseTable::generate(num_tickets, &customers);
    println!("Generating {num_compatible_parts} compatible parts");
    let compatible_parts =
        CompatiblePartsDatabaseJunctionTable::generate(num_compatible_parts, &devices, &parts);
    println!("Generating {num_ticket_devices} ticket devices");
    let ticket_devices =
        TicketDevicesDatabaseJunctionTable::generate(num_ticket_devices, &devices, &tickets);
    println!("Generating {num_bundled_parts} bundled parts");
    let bundled_parts =
        BundledPartsDatabaseJunctionTable::generate(num_bundled_parts, &tickets, &devices, &parts);

    let start_time = Instant::now();
    server_state
        .database
        .add_items(
            bundled_parts,
            compatible_parts,
            customers,
            device_categories,
            device_manufacturers,
            device_models,
            devices,
            part_categories,
            part_manufacturers,
            parts,
            ticket_devices,
            tickets,
            vendors,
        )
        .await;

    println!(
        "Inserted {} items in {}ms",
        (num_customers + num_tickets),
        start_time.elapsed().as_millis()
    );

    let cors = CorsLayer::new()
        .allow_methods([Method::GET, Method::POST])
        .allow_origin(Any);

    let routes = Router::new()
        .route("/customers", get(CustomersApiView::serve_json))
        .route("/tickets", get(TicketsApiView::serve_json))
        .layer(cors)
        .with_state(server_state);

    let listener = TcpListener::bind("127.0.0.1:8080").await.unwrap();
    println!("Listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, routes.into_make_service())
        .await
        .unwrap();
}
