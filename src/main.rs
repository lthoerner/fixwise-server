// #![allow(unused)]

mod database;

use std::collections::HashSet;
use std::io::Write;

use axum::response::Json;
use axum::routing::get;
use axum::Router;
use database::FrontendTableView;
use http::Method;
use rand::thread_rng;
use rand::Rng;
use tokio::net::TcpListener;
use tower_http::cors::{Any, CorsLayer};

use database::customers::Customer;
use database::inventory::InventoryItem;

#[tokio::main]
async fn main() {
    database::connect().await;

    let mut inventory_items = Vec::new();
    let mut existing = HashSet::new();
    let inventory_item_count = 123456;

    let loading_bar_size = 20;

    let mut previous_print_percent = 0.0;
    let mut percent;
    println!("Creating {} dummy inventory items", inventory_item_count);
    print!("[{}]", " ".repeat(loading_bar_size));
    print!("\x1B[2G");
    std::io::stdout().flush().unwrap();

    for i in 1..=inventory_item_count {
        percent = i as f32 * 100.0 / inventory_item_count as f32;
        let normalized_percent = percent.ceil();
        if normalized_percent - previous_print_percent == (100 / loading_bar_size) as f32
            && percent != 100.0
        {
            previous_print_percent = normalized_percent;
            print!("=");
            std::io::stdout().flush().unwrap();
        }

        inventory_items.push(InventoryItem::generate(&mut existing));
    }

    println!();

    let mut customers = Vec::new();
    existing.clear();
    let customer_count = 123456;

    previous_print_percent = 0.0;
    println!("Creating {} dummy customers", customer_count);
    print!("[{}]", " ".repeat(loading_bar_size));
    print!("\x1B[2G");

    for i in 1..=customer_count {
        percent = i as f32 * 100.0 / customer_count as f32;
        let normalized_percent = percent.ceil();
        if normalized_percent - previous_print_percent == (100 / loading_bar_size) as f32
            && percent != 100.0
        {
            previous_print_percent = normalized_percent;
            print!("=");
            std::io::stdout().flush().unwrap();
        }

        customers.push(Customer::generate(&mut existing));
    }

    println!();

    let start_time = std::time::Instant::now();
    database::add_items(&inventory_items, &customers).await;

    println!(
        "Inserted {} items in {}ms",
        (inventory_item_count + customer_count),
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

async fn get_inventory_view() -> Json<FrontendTableView> {
    database::get_frontend_view("inventory")
}

async fn get_customers_view() -> Json<FrontendTableView> {
    database::get_frontend_view("customers")
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
