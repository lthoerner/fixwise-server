mod api;
mod database;

use std::sync::Arc;

use axum::routing::get;
use axum::Router;
use http::Method;
use tokio::net::TcpListener;
use tokio::signal;
use tower_http::cors::{Any, CorsLayer};

use api::views::customers::CustomersApiView;
use api::views::device_models::DeviceModelsApiView;
use api::views::devices::DevicesApiView;
use api::views::parts::PartsApiView;
use api::views::tickets::TicketsApiView;
use api::views::vendors::VendorsApiView;
use api::ServeJson;
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
        .route("/customers", get(CustomersApiView::serve_json))
        .route("/device_models", get(DeviceModelsApiView::serve_json))
        .route("/devices", get(DevicesApiView::serve_json))
        .route("/parts", get(PartsApiView::serve_json))
        .route("/tickets", get(TicketsApiView::serve_json))
        .route("/vendors", get(VendorsApiView::serve_json))
        .layer(cors)
        .with_state(server_state);

    let listener = TcpListener::bind("127.0.0.1:8080").await.unwrap();
    println!("Listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, routes.into_make_service())
        .await
        .unwrap();
}
