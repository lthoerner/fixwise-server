#![allow(unused)]

mod database;

use std::io::Write;
use std::sync::OnceLock;

use axum::response::Json;
use axum::routing::get;
use axum::Router;
use database::FrontendTableView;
use fake::faker::address::en::{CityName, StateAbbr, StreetName, StreetSuffix};
use fake::faker::internet::en::FreeEmail;
use fake::faker::name::en::Name;
use fake::faker::phone_number::en::PhoneNumber;
use fake::{Dummy, Fake, Faker};
use http::Method;
use rand::thread_rng;
use rand::Rng;
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use tokio::net::TcpListener;
use tokio_postgres::{Client, Config, NoTls};
use tower_http::cors::{Any, CorsLayer};

#[derive(Debug, Clone, Serialize, Deserialize)]
struct InventoryItem {
    sku: i64,
    display_name: String,
    count: i64,
    cost: Decimal,
    price: Decimal,
}

#[derive(Debug, Clone, Serialize, Deserialize, Dummy)]
struct Customer {
    id: i32,
    #[dummy(faker = "Name()")]
    name: String,
    #[dummy(faker = "FreeEmail()")]
    email: String,
    #[dummy(faker = "PhoneNumber()")]
    phone: String,
    address: String,
}

#[derive(Dummy)]
struct StreetAddress {
    #[dummy(faker = "1..10000")]
    number: u32,
    #[dummy(faker = "StreetName()")]
    street: String,
    #[dummy(faker = "StreetSuffix()")]
    street_suffix: String,
    #[dummy(faker = "CityName()")]
    city: String,
    #[dummy(faker = "StateAbbr()")]
    state: String,
    #[dummy(faker = "10000..99999")]
    zip: u32,
}

impl Customer {
    fn generate() -> Self {
        let address: StreetAddress = Faker.fake();
        let customer: Customer = Faker.fake();

        Customer {
            id: generate_random_i32(1000000000),
            address: format!(
                "{} {} {}, {}, {} {}",
                address.number,
                address.street,
                address.street_suffix,
                address.city,
                address.state,
                address.zip
            ),
            ..customer
        }
    }

    fn build_query(&self) -> String {
        format!(
            "INSERT INTO customers (id, name, email, phone, address) VALUES ({}, $1, $2, $3, $4)",
            self.id
        )
    }
}

fn generate_random_i32(min: i32) -> i32 {
    thread_rng().gen_range(min..=i32::MAX)
}

fn generate_random_i64(min: i64) -> i64 {
    thread_rng().gen_range(min..=i64::MAX)
}

static DB: OnceLock<Client> = OnceLock::new();

macro_rules! get_db {
    () => {
        DB.get().unwrap()
    };
}

#[tokio::main]
async fn main() {
    let mut connection_config = Config::new();
    connection_config
        .user("techtriage")
        .password("techtriage")
        .host("localhost")
        .port(54206);

    let (client, connection) = connection_config.connect(NoTls).await.unwrap();

    DB.get_or_init(|| client);

    tokio::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("Connection error: {e}");
        }
    });

    let setup_script = database::create_setup_script();
    println!("{setup_script}");
    get_db!().batch_execute(&setup_script).await.unwrap();

    let mut inventory_items = Vec::new();
    let inventory_item_count = 1234;

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

        inventory_items.push(InventoryItem::generate(&inventory_items));
    }

    println!();

    let mut customers = Vec::new();
    let customer_count = 1234;

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

        customers.push(Customer::generate());
    }

    println!();

    let start_time = std::time::Instant::now();
    for item in inventory_items {
        get_db!().query(&item.build_query(), &[]).await.unwrap();
    }

    for customer in customers {
        get_db!()
            .query(
                &customer.build_query(),
                &[
                    &customer.name,
                    &customer.email,
                    &customer.phone,
                    &customer.address,
                ],
            )
            .await
            .unwrap();
    }

    println!(
        "Inserted {} items in {}ms",
        (inventory_item_count + customer_count),
        start_time.elapsed().as_millis()
    );

    let cors = CorsLayer::new()
        .allow_methods([Method::GET, Method::POST])
        .allow_origin(Any);

    let routes = Router::new()
        .route("/inventory", get(get_inventory))
        .route("/views/inventory", get(get_inventory_view))
        .route("/customers", get(get_customers))
        .route("/views/customers", get(get_customers_view))
        .layer(cors);

    let listener = TcpListener::bind("127.0.0.1:8080").await.unwrap();
    println!("Listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, routes.into_make_service())
        .await
        .unwrap();
}

async fn get_inventory() -> Json<Vec<InventoryItem>> {
    let inventory_rows = get_db!()
        .query("SELECT * FROM inventory ORDER BY sku", &[])
        .await
        .unwrap();

    let mut inventory_items = Vec::new();
    for item in inventory_rows {
        inventory_items.push(InventoryItem {
            sku: item.get::<_, i32>("sku") as i64,
            display_name: item.get::<_, String>("display_name"),
            count: item.get::<_, i32>("count") as i64,
            cost: item.get::<_, Decimal>("cost"),
            price: item.get::<_, Decimal>("price"),
        });
    }

    Json(inventory_items)
}

async fn get_customers() -> Json<Vec<Customer>> {
    let customer_rows = get_db!()
        .query("SELECT * FROM customers ORDER BY id", &[])
        .await
        .unwrap();

    let mut customers = Vec::new();
    for customer in customer_rows {
        customers.push(Customer {
            id: customer.get::<_, i32>("id"),
            name: customer.get::<_, String>("name"),
            email: customer.get::<_, String>("email"),
            phone: customer.get::<_, String>("phone"),
            address: customer.get::<_, String>("address"),
        });
    }

    Json(customers)
}

async fn get_inventory_view() -> Json<FrontendTableView> {
    database::get_frontend_view("inventory")
}

async fn get_customers_view() -> Json<FrontendTableView> {
    database::get_frontend_view("customers")
}

impl InventoryItem {
    fn generate(existing_items: &[Self]) -> Self {
        let mut sku: i64 = 0;
        let mut first_roll = true;
        while first_roll || existing_items.iter().any(|item| item.sku == sku) {
            sku = thread_rng().gen_range(0..=99999999);
            first_roll = false;
        }

        let count: i64 = thread_rng().gen_range(1..=9999);
        let cost = Decimal::new(thread_rng().gen_range(10000..=999999), 2);
        let price = cost * Decimal::new(thread_rng().gen_range(2..=5), 0);

        InventoryItem {
            sku,
            display_name: Self::generate_display_name(),
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

    fn build_query(&self) -> String {
        format!(
            "INSERT INTO inventory (sku, display_name, count, cost, price) VALUES ({}, '{}', {}, {}, {})",
            self.sku, self.display_name, self.count, self.cost, self.price
        )
    }
}
