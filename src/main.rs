// #![allow(unused)]

mod database;

use std::collections::HashSet;
use std::io::Write;
use std::time::Instant;

use axum::routing::get;
use axum::Router;
use database::views::ViewConfigurations;
use http::Method;
use rand::thread_rng;
use rand::Rng;
use tokio::net::TcpListener;
// use tokio::signal;
use tower_http::cors::{Any, CorsLayer};

use database::customers::Customer;
use database::inventory::InventoryItem;
use database::tickets::Ticket;
use database::Database;

#[derive(Clone)]
struct ServerState {
    database: Database,
    view_configurations: ViewConfigurations,
}

#[tokio::main]
async fn main() {
    let server_state = ServerState {
        database: Database::connect_and_configure().await,
        view_configurations: ViewConfigurations::load(),
    };

    // tokio::spawn(async {
    //     signal::ctrl_c().await.unwrap();
    //     println!();
    //     println!("Server shutting down...");
    //     get_db!().close().await;
    //     println!("Database connection closed.");
    //     std::process::exit(0);
    // });

    let bar_length = 33;
    let num_inventory_items = 1234;
    let num_customers = 1234;
    let num_tickets = 1234;
    println!("Generating {num_inventory_items} inventory items");
    let inventory_items = generate_items(bar_length, num_inventory_items, InventoryItem::generate);
    println!("Generating {num_customers} customers");
    let customers = generate_items(bar_length, num_customers, Customer::generate);
    println!("Generating {num_tickets} tickets");
    let tickets = generate_tickets(bar_length, num_tickets, &customers, Ticket::generate);

    let start_time = Instant::now();
    server_state
        .database
        .add_items(inventory_items, customers, tickets)
        .await;

    println!(
        "Inserted {} items in {}ms",
        (num_inventory_items + num_customers + num_tickets),
        start_time.elapsed().as_millis()
    );

    let cors = CorsLayer::new()
        .allow_methods([Method::GET, Method::POST])
        .allow_origin(Any);

    let routes = Router::new()
        .route("/inventory", get(database::inventory::get_inventory))
        .route("/customers", get(database::customers::get_customers))
        .route("/tickets", get(database::tickets::get_tickets))
        // TODO: Maybe reconsider the routing here
        .route("/views/inventory", get(database::views::get_inventory_view))
        .route("/views/customers", get(database::views::get_customers_view))
        .route("/views/tickets", get(database::views::get_tickets_view))
        .layer(cors)
        .with_state(server_state);

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

fn generate_tickets<T>(
    bar_length: usize,
    count: usize,
    customers: &[Customer],
    add_function: impl Fn(&mut HashSet<i32>, &[Customer]) -> T,
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

        elements.push(add_function(&mut existing, customers));
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
