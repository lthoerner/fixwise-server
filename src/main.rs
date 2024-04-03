mod api;
mod database;

use std::collections::{HashMap, HashSet};
use std::io::Write;
use std::time::Instant;

use axum::routing::get;
use axum::Router;
use http::Method;
use rand::thread_rng;
use rand::Rng;
use tokio::net::TcpListener;
// use tokio::signal;
use tower_http::cors::{Any, CorsLayer};

use api::views::customers::CustomersApiView;
use api::views::inventory::InventoryApiView;
use api::views::tickets::TicketsApiView;
use api::ServeJson;
use database::tables::{Generate, IdentifiableRow};
use database::Database;

#[derive(Clone)]
struct ServerState {
    database: Database,
}

#[tokio::main]
async fn main() {
    let server_state = ServerState {
        database: Database::connect_and_configure().await,
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
    let num_inventory_items = 12345;
    let num_customers = 12345;
    let num_tickets = 12345;

    let mut dependencies = HashMap::new();

    println!("Generating {num_inventory_items} inventory items");
    let inventory_items = generate_items(bar_length, num_inventory_items, &dependencies);

    println!("Generating {num_customers} customers");
    let customers = generate_items(bar_length, num_customers, &dependencies);
    dependencies.insert("customers", &customers);

    println!("Generating {num_tickets} tickets");
    let tickets = generate_items(bar_length, num_tickets, &dependencies);

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
        .route("/inventory", get(InventoryApiView::serve_json))
        .route("/customers", get(CustomersApiView::serve_json))
        .route("/tickets", get(TicketsApiView::serve_json))
        .layer(cors)
        .with_state(server_state);

    let listener = TcpListener::bind("127.0.0.1:8080").await.unwrap();
    println!("Listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, routes.into_make_service())
        .await
        .unwrap();
}

fn generate_items<'a, T: Generate>(
    bar_length: usize,
    count: usize,
    dependencies: &'a HashMap<&'static str, &'a [impl IdentifiableRow]>,
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

        elements.push(T::generate(&mut existing, dependencies));
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
