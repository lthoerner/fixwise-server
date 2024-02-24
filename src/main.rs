#![allow(unused)]

use std::future::{Future, IntoFuture};
use std::sync::OnceLock;

use axum::response::{Html, IntoResponse, Json};
use axum::routing::get;
use axum::Router;
use serde::{Deserialize, Serialize};
use surrealdb::engine::local::{Db, Mem};
// use surrealdb::opt::auth::Root;
use surrealdb::{sql, Surreal};
use tokio::net::TcpListener;

const NBSP: char = '\u{00A0}';

#[derive(Debug, Serialize, Deserialize)]
struct InventoryItem {
    id: sql::Thing,
    sku: i64,
    display_name: String,
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

    let routes = Router::new().route("/inventory", get(query_inventory));

    let listener = TcpListener::bind("127.0.0.1:8080").await.unwrap();
    println!("Listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, routes.into_make_service())
        .await
        .unwrap();
}

async fn query_inventory() -> Json<Vec<InventoryItem>> {
    let inventory_items: Vec<InventoryItem> = get_db!()
        .query("SELECT * FROM inventory")
        .await
        .unwrap()
        .take(0)
        .unwrap();

    Json(inventory_items)
}
