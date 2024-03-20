// #![allow(unused)]

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

#[derive(Debug, Clone, Serialize, Deserialize)]
struct DatabaseConfig {
    tables: Vec<TableConfig>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct TableConfig {
    name: String,
    columns: Vec<ColumnConfig>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct ColumnConfig {
    name: String,
    display_name: String,
    data_type: String,
    #[serde(skip_serializing)]
    required: Option<bool>,
    formatting: Option<ColumnFormattingConfig>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct ColumnFormattingConfig {
    prefix: Option<String>,
    suffix: Option<String>,
    pad_length: Option<i64>,
}

static DB: OnceLock<Client> = OnceLock::new();
static CONFIG: OnceLock<DatabaseConfig> = OnceLock::new();

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
        .port(57589);

    let (client, connection) = connection_config.connect(NoTls).await.unwrap();

    DB.get_or_init(|| client);

    tokio::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("Connection error: {e}");
        }
    });

    let setup_script = create_setup_script();
    println!("{setup_script}");
    get_db!().batch_execute(&setup_script).await.unwrap();

    let mut inventory_items = Vec::new();
    let items = 3;

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

        inventory_items.push(InventoryItem::generate(&inventory_items));
    }

    println!();

    let start_time = std::time::Instant::now();
    for item in inventory_items {
        get_db!().query(&item.into_query(), &[]).await.unwrap();
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
        .route("/inventory/schema", get(get_inventory_schema))
        .layer(cors);

    let listener = TcpListener::bind("127.0.0.1:8080").await.unwrap();
    println!("Listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, routes.into_make_service())
        .await
        .unwrap();
}

fn create_setup_script() -> String {
    let config: DatabaseConfig = toml::from_str(include_str!("../database/schema.toml")).unwrap();

    let mut script = String::new();

    for table in &config.tables {
        script.push_str(&format!("DROP TABLE IF EXISTS {} CASCADE;\n", table.name));
    }

    for table in &config.tables {
        let column_declarations = table
            .columns
            .iter()
            .enumerate()
            .map(|(i, col)| generate_column(col, i == 0))
            .collect::<Vec<String>>();

        script.push_str(&format!(
            "\nCREATE TABLE {} (\n{}\n);\n",
            table.name,
            column_declarations.join(",\n")
        ));
    }

    fn generate_column(column: &ColumnConfig, primary_column: bool) -> String {
        let name = column.name.to_owned();
        let data_type = map_type(&column.data_type, primary_column);
        let required = column.required.unwrap_or(true);

        format!(
            "    {:<16}{}{}",
            name,
            data_type,
            if primary_column {
                " PRIMARY KEY"
            } else if required {
                " NOT NULL"
            } else {
                ""
            }
        )
        .to_owned()
    }

    fn map_type(type_name: &str, primary_column: bool) -> String {
        if primary_column && type_name == "integer" {
            return "serial".to_owned();
        }

        match type_name {
            "integer" => "integer",
            "decimal" => "numeric(1000, 2)",
            "string" => "text",
            _ => panic!("Unknown type in schema config"),
        }
        .to_owned()
    }

    CONFIG.get_or_init(|| config);

    script
}

async fn query_inventory() -> Json<Vec<InventoryItem>> {
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

async fn get_inventory_schema() -> Json<TableConfig> {
    get_schema("inventory")
}

fn get_schema(table: &str) -> Json<TableConfig> {
    let schema = CONFIG.get().unwrap();
    let table = schema.tables.iter().find(|t| t.name == table).unwrap();

    Json(table.to_owned())
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

    fn into_query(self) -> String {
        format!(
            "INSERT INTO inventory (sku, display_name, count, cost, price) VALUES ({}, '{}', {}, {}, {})",
            self.sku, self.display_name, self.count, self.cost, self.price
        )
    }
}
