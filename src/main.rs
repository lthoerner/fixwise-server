mod api;
mod database;

use std::sync::Arc;

use axum::routing::get;
use axum::Router;
use http::Method;
use tokio::net::TcpListener;
use tokio::signal;
use tower_http::cors::{Any, CorsLayer};

use api::endpoints::formatted::customers::CustomersApiEndpoint;
use api::endpoints::formatted::device_models::DeviceModelsApiEndpoint;
use api::endpoints::formatted::devices::DevicesApiEndpoint;
use api::endpoints::formatted::parts::PartsApiEndpoint;
use api::endpoints::formatted::tickets::TicketsApiEndpoint;
use api::endpoints::formatted::vendors::VendorsApiEndpoint;
use api::endpoints::raw::invoices::InvoicesApiEndpoint;
use api::endpoints::utils::imei_check::ImeiInfoApiUtil;
use api::{ServeEntityJson, ServeRowJson};
use database::Database;

#[derive(Clone)]
struct ServerState {
    database: Database,
    imei_info_api_key: String,
}

#[tokio::main]
async fn main() {
    dotenvy::dotenv().unwrap();

    println!("Connecting to database...");
    let server_state = Arc::new(ServerState {
        database: Database::connect_and_configure().await,
        imei_info_api_key: std::env::var("IMEI_INFO_API_KEY").unwrap(),
    });

    let signal_handler_server_state = server_state.clone();
    tokio::spawn(async move {
        signal::ctrl_c().await.unwrap();
        println!();
        println!("Server shutting down...");
        signal_handler_server_state
            .database
            .close_connection()
            .await;
        println!("Database connection closed.");
        std::process::exit(0);
    });

    server_state.database.add_generated_items().await;

    let cors = CorsLayer::new()
        .allow_methods([Method::GET, Method::POST])
        .allow_origin(Any);

    let routes = Router::new()
        .route("/customers", get(CustomersApiEndpoint::serve_all))
        .route("/device_models", get(DeviceModelsApiEndpoint::serve_all))
        .route("/devices", get(DevicesApiEndpoint::serve_all))
        .route("/parts", get(PartsApiEndpoint::serve_all))
        .route("/tickets", get(TicketsApiEndpoint::serve_all))
        .route("/vendors", get(VendorsApiEndpoint::serve_all))
        .route("/imei_check", get(ImeiInfoApiUtil::serve_one))
        .route("/raw/invoices", get(InvoicesApiEndpoint::serve_all))
        .layer(cors)
        .with_state(server_state);

    let listener = TcpListener::bind("0.0.0.0:8080").await.unwrap();
    println!("Listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, routes.into_make_service())
        .await
        .unwrap();
}
