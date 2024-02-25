use std::sync::OnceLock;

use axum::response::Json;
use axum::routing::get;
use axum::Router;
use http::Method;
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use surrealdb::engine::local::{Db, Mem};
use surrealdb::{sql, Surreal};
use tokio::net::TcpListener;
use tower_http::cors::{Any, CorsLayer};

#[derive(Debug, Serialize, Deserialize)]
struct InventoryItem {
    id: sql::Thing,
    display_name: String,
    sku: i64,
    count: i64,
    cost: Decimal,
    price: Decimal,
}

static DB: OnceLock<Surreal<Db>> = OnceLock::new();

macro_rules! get_db {
    () => {
        DB.get().unwrap()
    };
}

#[tokio::main]
async fn main() {
    let db = Surreal::new::<Mem>(()).await.unwrap();
    db.use_ns("test").use_db("test").await.unwrap();
    DB.get_or_init(|| db);

    let statements = include_str!("../database/setup.surql");
    for statement in statements.lines() {
        if !statement.is_empty() {
            get_db!().query(statement).await.unwrap();
        }
    }

    let cors = CorsLayer::new()
        .allow_methods([Method::GET, Method::POST])
        .allow_origin(Any);

    let routes = Router::new()
        .route("/inventory", get(query_inventory))
        .layer(cors);

    let listener = TcpListener::bind("127.0.0.1:8080").await.unwrap();
    println!("Listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, routes.into_make_service())
        .await
        .unwrap();
}

async fn query_inventory() -> Json<Vec<InventoryItem>> {
    let inventory_items: Vec<InventoryItem> = get_db!()
        .query("SELECT * FROM inventory ORDER BY sku ASC")
        .await
        .unwrap()
        .take(0)
        .unwrap();

    println!("{:#?}", inventory_items);

    Json(inventory_items)
}
