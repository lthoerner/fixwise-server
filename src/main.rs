mod api;
mod database;

use std::sync::Arc;

use axum::routing::{delete, get, post};
use axum::Router;
use crudkit::traits::id_parameter::GenericIdParameter;
use http::Method;
use tokio::net::TcpListener;
use tokio::signal;
use tower_http::cors::{Any, CorsLayer};

use api::endpoints::processed::{
    CustomersResource, DeviceModelsResource, DevicesResource, InvoicesResource, PartsResource,
    ProductsResource, ServicesResource, TicketsResource, VendorsResource,
};
use api::endpoints::utils::ImeiInfoApiUtil;
use api::{ServeRecordJson, ServeResourceJson};
use crudkit::database::DatabaseState;
use crudkit::traits::read::ReadRelation;
use crudkit::traits::write::WriteRelation;
use database::tables::{
    CustomersTable, DeviceModelsTable, DevicesTable, InvoicesTable, ItemsTable, PartsTable,
    ProductsTable, ServicesTable, TicketsTable, VendorsTable,
};
use database::views::{
    CustomersView, DeviceModelsView, DevicesView, InvoicesView, ItemsView, PartsView, ProductsView,
    ServicesView, TicketsView, VendorsView,
};
use database::Database;

#[derive(Clone)]
struct ServerState {
    database: Database,
    imei_info_api_key: String,
}

impl DatabaseState for ServerState {
    fn get_database(&self) -> &crudkit::database::PgDatabase {
        &self.database.0
    }

    fn get_database_connection(&self) -> &sqlx::PgPool {
        &self.database.0.connection
    }
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

    server_state.database.add_generated_items().await.unwrap();

    let cors = CorsLayer::new()
        .allow_methods([Method::GET, Method::POST, Method::DELETE])
        .allow_origin(Any);

    let routes = Router::new()
        .route("/imei_check", get(ImeiInfoApiUtil::serve_one))
        .route("/customers", get(CustomersResource::serve_all))
        .route("/device_models", get(DeviceModelsResource::serve_all))
        .route("/devices", get(DevicesResource::serve_all))
        .route("/invoices", get(InvoicesResource::serve_all))
        .route("/parts", get(PartsResource::serve_all))
        .route("/products", get(ProductsResource::serve_all))
        .route("/services", get(ServicesResource::serve_all))
        .route("/tickets", get(TicketsResource::serve_all))
        .route("/vendors", get(VendorsResource::serve_all))
        .route("/raw/customers", get(CustomersView::query_all_handler))
        .route(
            "/raw/device_models",
            get(DeviceModelsView::query_all_handler),
        )
        .route("/raw/devices", get(DevicesView::query_all_handler))
        .route("/raw/invoices", get(InvoicesView::query_all_handler))
        .route("/raw/items", get(ItemsView::query_all_handler))
        .route("/raw/parts", get(PartsView::query_all_handler))
        .route("/raw/products", get(ProductsView::query_all_handler))
        .route("/raw/services", get(ServicesView::query_all_handler))
        .route("/raw/tickets", get(TicketsView::query_all_handler))
        .route("/raw/vendors", get(VendorsView::query_all_handler))
        .route(
            "/raw/customers/delete",
            delete(CustomersTable::delete_one_handler::<GenericIdParameter, ServerState>),
        )
        .route(
            "/raw/device_models/delete",
            delete(DeviceModelsTable::delete_one_handler::<GenericIdParameter, ServerState>),
        )
        .route(
            "/raw/devices/delete",
            delete(DevicesTable::delete_one_handler::<GenericIdParameter, ServerState>),
        )
        .route(
            "/raw/invoices/delete",
            delete(InvoicesTable::delete_one_handler::<GenericIdParameter, ServerState>),
        )
        .route(
            "/raw/items/delete",
            delete(ItemsTable::delete_one_handler::<GenericIdParameter, ServerState>),
        )
        .route(
            "/raw/parts/delete",
            delete(PartsTable::delete_one_handler::<GenericIdParameter, ServerState>),
        )
        .route(
            "/raw/products/delete",
            delete(ProductsTable::delete_one_handler::<GenericIdParameter, ServerState>),
        )
        .route(
            "/raw/services/delete",
            delete(ServicesTable::delete_one_handler::<GenericIdParameter, ServerState>),
        )
        .route(
            "/raw/tickets/delete",
            delete(TicketsTable::delete_one_handler::<GenericIdParameter, ServerState>),
        )
        .route(
            "/raw/vendors/delete",
            delete(VendorsTable::delete_one_handler::<GenericIdParameter, ServerState>),
        )
        .route(
            "/raw/customers/create",
            post(CustomersTable::create_one_handler),
        )
        .route(
            "/raw/device_models/create",
            post(DeviceModelsTable::create_one_handler),
        )
        .route(
            "/raw/devices/create",
            post(DevicesTable::create_one_handler),
        )
        .route(
            "/raw/invoices/create",
            post(InvoicesTable::create_one_handler),
        )
        .route("/raw/items/create", post(ItemsTable::create_one_handler))
        .route("/raw/parts/create", post(PartsTable::create_one_handler))
        .route(
            "/raw/products/create",
            post(ProductsTable::create_one_handler),
        )
        .route(
            "/raw/services/create",
            post(ServicesTable::create_one_handler),
        )
        .route(
            "/raw/tickets/create",
            post(TicketsTable::create_one_handler),
        )
        .route(
            "/raw/vendors/create",
            post(VendorsTable::create_one_handler),
        )
        .route(
            "/raw/customers/update",
            get(CustomersTable::update_one_handler),
        )
        .layer(cors)
        .with_state(server_state);

    let listener = TcpListener::bind("0.0.0.0:8080").await.unwrap();
    println!("Listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, routes.into_make_service())
        .await
        .unwrap();
}
