mod api;
mod database;

use std::sync::Arc;

use axum::routing::{delete, get};
use axum::Router;
use database::views::invoices::InvoicesView;
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
use api::endpoints::utils::imei_check::ImeiInfoApiUtil;
use api::{GenericIdParameter, ServeRecordJson, ServeResourceJson};
use database::tables::invoices::InvoicesTable;
use database::tables::tickets::TicketsTable;
use database::views::items::ItemsView;
use database::views::products::ProductsView;
use database::views::services::ServicesView;
use database::views::tickets::TicketsView;
use database::{Database, Relation, Table};

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
        .route("/raw/products", get(ProductsView::query_all_handler))
        .route("/raw/services", get(ServicesView::query_all_handler))
        .route("/raw/items", get(ItemsView::query_all_handler))
        .route("/raw/tickets", get(TicketsView::query_all_handler))
        .route("/raw/invoices", get(InvoicesView::query_all_handler))
        .route(
            "/raw/invoices/delete",
            delete(InvoicesTable::delete_one_handler::<GenericIdParameter>),
        )
        .route(
            "/raw/tickets/delete",
            delete(TicketsTable::delete_one_handler::<GenericIdParameter>),
        )
        .layer(cors)
        .with_state(server_state);

    let listener = TcpListener::bind("0.0.0.0:8080").await.unwrap();
    println!("Listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, routes.into_make_service())
        .await
        .unwrap();
}
