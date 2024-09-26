mod api;
mod database;

use std::sync::Arc;

use axum::routing::get;
use axum::Router;
use http::Method;
use tokio::net::TcpListener;
use tokio::signal;
use tower_http::cors::{Any, CorsLayer};

use api::endpoints::processed::customers::CustomersResource;
use api::endpoints::processed::device_models::DeviceModelsResource;
use api::endpoints::processed::devices::DevicesResource;
use api::endpoints::processed::invoices::InvoicesResource;
use api::endpoints::processed::parts::PartsResource;
use api::endpoints::processed::products::ProductsResource;
use api::endpoints::processed::services::ServicesResource;
use api::endpoints::processed::tickets::TicketsResource;
use api::endpoints::processed::vendors::VendorsResource;
use api::endpoints::raw::invoices::InvoicesResource as InvoicesRawResource;
use api::endpoints::raw::items::ItemsResource;
use api::endpoints::raw::products::ProductsResource as ProductsRawResource;
use api::endpoints::raw::services::ServicesResource as ServicesRawResource;
use api::endpoints::utils::imei_check::ImeiInfoApiUtil;
use api::{ServeRecordJson, ServeResourceJson};
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
        .allow_methods([Method::GET, Method::DELETE])
        .allow_origin(Any);

    let routes = Router::new()
        .route("/customers", get(CustomersResource::serve_all))
        .route("/device_models", get(DeviceModelsResource::serve_all))
        .route("/devices", get(DevicesResource::serve_all))
        .route("/parts", get(PartsResource::serve_all))
        .route("/tickets", get(TicketsResource::serve_all))
        .route("/invoices", get(InvoicesResource::serve_all))
        .route("/vendors", get(VendorsResource::serve_all))
        .route("/products", get(ProductsResource::serve_all))
        .route("/services", get(ServicesResource::serve_all))
        .route("/imei_check", get(ImeiInfoApiUtil::serve_one))
        .route("/raw/products", get(ProductsRawResource::serve_all))
        .route("/raw/services", get(ServicesRawResource::serve_all))
        .route("/raw/items", get(ItemsResource::serve_all))
        .route("/raw/invoices", get(InvoicesRawResource::serve_all))
        .layer(cors)
        .with_state(server_state);

    let listener = TcpListener::bind("0.0.0.0:8080").await.unwrap();
    println!("Listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, routes.into_make_service())
        .await
        .unwrap();
}
