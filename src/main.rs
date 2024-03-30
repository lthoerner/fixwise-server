// #![allow(unused)]

mod database;

use std::collections::HashSet;
use std::fs::File;
use std::io::BufReader;
use std::io::Write;

use axum::response::Json;
use axum::routing::get;
use axum::Router;
use http::Method;
use rand::thread_rng;
use rand::Rng;
use tokio::net::TcpListener;
use tower_http::cors::{Any, CorsLayer};

use database::config::FrontendTableView;
use database::customers::Customer;
use database::inventory::InventoryItem;

#[tokio::main]
async fn main() {
    database::connect().await;

    let bar_length = 20;
    let num_inventory_items = 123456;
    let num_customers = 123456;
    println!("Generating {num_inventory_items} inventory items");
    let inventory_items = generate_items(bar_length, num_inventory_items, InventoryItem::generate);
    println!("Generating {num_customers} customers");
    let customers = generate_items(bar_length, num_customers, Customer::generate);

    let start_time = std::time::Instant::now();
    database::add_items(inventory_items, customers).await;

    println!(
        "Inserted {} items in {}ms",
        (num_inventory_items + num_customers),
        start_time.elapsed().as_millis()
    );

    let cors = CorsLayer::new()
        .allow_methods([Method::GET, Method::POST])
        .allow_origin(Any);

    let routes = Router::new()
        .route("/inventory", get(database::inventory::get_inventory))
        .route("/customers", get(database::customers::get_customers))
        .route("/views/inventory", get(get_inventory_view))
        .route("/views/customers", get(get_customers_view))
        .layer(cors);

    let listener = TcpListener::bind("127.0.0.1:8080").await.unwrap();
    println!("Listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, routes.into_make_service())
        .await
        .unwrap();
}

fn generate_items<T>(
    bar_length: usize,
    count: usize,
    add_function: impl Fn(&mut HashSet<i32>) -> T,
) -> Vec<T> {
    let mut existing = HashSet::new();
    let mut elements = Vec::new();

    let mut percent;
    let mut previous_print_percent = 0.0;
    print!("[{}]", " ".repeat(bar_length));
    print!("\x1B[2G");

    for i in 1..=count {
        percent = i as f32 * 100.0 / count as f32;
        let normalized_percent = percent.ceil();
        if normalized_percent - previous_print_percent == (100 / bar_length) as f32
            && percent != 100.0
        {
            previous_print_percent = normalized_percent;
            print!("=");
            std::io::stdout().flush().unwrap();
        }

        elements.push(add_function(&mut existing));
    }

    println!();

    elements
}

fn generate_unique_random_i32(min: i32, existing: &mut HashSet<i32>) -> i32 {
    let mut val = 0;
    let mut first_roll = true;
    while first_roll || existing.get(&val).is_some() {
        val = thread_rng().gen_range(min..=i32::MAX);
        first_roll = false;
    }

    existing.insert(val);

    val
}

async fn get_inventory_view() -> Json<FrontendTableView> {
    get_json_file("./database/frontend_views/inventory.json")
}

async fn get_customers_view() -> Json<FrontendTableView> {
    get_json_file("./database/frontend_views/customers.json")
}

fn get_json_file(filename: &str) -> Json<FrontendTableView> {
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);
    Json(serde_json::from_reader(reader).unwrap())
}
