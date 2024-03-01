use std::sync::OnceLock;

use axum::extract::Query;
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

#[derive(Debug, Deserialize)]
struct InventoryOrderParams {
    column: Option<String>,
    direction: Option<String>,
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
        if !statement.is_empty() && !statement.starts_with("--") {
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

async fn query_inventory(Query(params): Query<InventoryOrderParams>) -> Json<Vec<InventoryItem>> {
    let column = match params.column.unwrap_or_default().to_lowercase().as_str() {
        "sku" => "sku",
        "display_name" => "display_name",
        "count" => "count",
        "cost" => "cost",
        "price" => "price",
        _ => "sku",
    };

    let direction = match params.direction.unwrap_or_default().to_lowercase().as_str() {
        "asc" => "ASC",
        "desc" => "DESC",
        _ => "ASC",
    };

    let inventory_items: Vec<InventoryItem> = get_db!()
        .query(format!(
            "SELECT * FROM inventory ORDER BY {} {}",
            column, direction
        ))
        .await
        .unwrap()
        .take(0)
        .unwrap();

    Json(inventory_items)
}
