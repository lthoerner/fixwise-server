use std::sync::OnceLock;

// use axum::extract::Query;
use axum::response::Json;
use axum::routing::get;
use axum::Router;
use http::Method;
use rand::thread_rng;
use rand::Rng;
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use surrealdb::engine::local::{Db, Mem};
use surrealdb::{sql, Surreal};
use tokio::net::TcpListener;
use tower_http::cors::{Any, CorsLayer};

#[derive(Debug, Serialize, Deserialize)]
struct InventoryItem {
    sku: i64,
    display_name: String,
    count: i64,
    cost: Decimal,
    price: Decimal,
}

#[derive(Debug, Serialize, Deserialize)]
struct TableSchema {
    table_fields: Vec<ColumnSchema>,
}

#[derive(Debug, Serialize, Deserialize)]
struct ColumnSchema {
    true_name: String,
    display_name: String,
    search_weight: i64,
    data_type: String,
    formatting: Option<ColumnFormatting>,
}

#[derive(Debug, Serialize, Deserialize)]
struct ColumnFormatting {
    prefix: Option<String>,
    suffix: Option<String>,
    pad_length: Option<i64>,
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

    let setup_script = include_str!("../database/setup.surql");
    get_db!().query(setup_script).await.unwrap();

    let mut inventory_items = Vec::new();
    let items = 123456;

    let loading_bar_size = 20;
    let mut previous_print_percent = 0.0;
    let mut percent;
    print!("[{}]", " ".repeat(loading_bar_size));
    print!("\x1B[2G");
    use std::io::Write;
    std::io::stdout().flush().unwrap();

    for i in 1..=items {
        percent = i as f32 * 100.0 / items as f32;
        let normalized_percent = percent.ceil();
        if normalized_percent - previous_print_percent == (100 / loading_bar_size) as f32
            && percent != 100.0
        {
            previous_print_percent = normalized_percent;
            print!("=");
            std::io::stdout().flush().unwrap();
        }

        inventory_items.push(generate_item(&inventory_items));
    }

    println!();

    let start_time = std::time::Instant::now();
    for item in inventory_items {
        get_db!().query(item.into_query()).await.unwrap();
        // println!("Item cost: {:?}", item.cost);
        // println!("Item price: {:?}", item.price);

        // let _: Vec<InventoryItem> = get_db!().create("inventory").content(item).await.unwrap();
    }

    println!(
        "Inserted {} items in {}ms",
        items,
        start_time.elapsed().as_millis()
    );

    let cors = CorsLayer::new()
        .allow_methods([Method::GET, Method::POST])
        .allow_origin(Any);

    let routes = Router::new()
        .route("/inventory", get(query_inventory))
        .route("/inventory/schema", get(query_inventory_schema))
        .layer(cors);

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

async fn query_inventory_schema() -> Json<TableSchema> {
    let inventory_schema: Option<TableSchema> = get_db!()
        .query("SELECT table_fields FROM schema:inventory")
        .await
        .unwrap()
        .take(0)
        .unwrap();

    Json(inventory_schema.unwrap())
}

fn generate_item(existing_items: &[InventoryItem]) -> InventoryItem {
    let mut sku: i64 = 0;
    let mut first_roll = true;
    while first_roll || existing_items.iter().any(|item| item.sku == sku) {
        sku = thread_rng().gen_range(0..=9999999);
        first_roll = false;
    }

    let count: i64 = thread_rng().gen_range(1..=9999);
    let cost = Decimal::new(thread_rng().gen_range(10000..=999999), 2);
    let price = cost * Decimal::new(thread_rng().gen_range(2..=5), 0);

    InventoryItem {
        sku,
        display_name: generate_display_name(),
        count,
        cost,
        price,
    }
}

fn generate_display_name() -> String {
    const PHONE_LINES: [&str; 8] = [
        "iPhone",
        "Samsung Galaxy",
        "Google Pixel",
        "Motorola G",
        "LG",
        "Nokia",
        "Sony Xperia",
        "OnePlus",
    ];

    const MODIFIERS: [&str; 8] = ["Pro", "Max", "Ultra", "Plus", "Lite", "Mini", "X", "Z"];

    let phone = PHONE_LINES[thread_rng().gen_range(0..PHONE_LINES.len())];
    let generation = thread_rng().gen_range(1..=50);
    let modifier = MODIFIERS[thread_rng().gen_range(0..MODIFIERS.len())];

    format!("{} {} {}", phone, generation, modifier)
}

impl InventoryItem {
    fn into_query(self) -> String {
        format!(
            "CREATE inventory SET sku = {}, display_name = '{}', count = {}, cost = {}, price = {}",
            self.sku, self.display_name, self.count, self.cost, self.price
        )
    }
}
