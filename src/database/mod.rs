pub mod config;
pub mod customers;
pub mod inventory;

use std::iter;
use std::sync::OnceLock;

use axum::Json;
use itertools::Itertools;
use serde::Serialize;
use sqlx::{Pool, Postgres, QueryBuilder};

use config::CONFIG;
use config::{ColumnFormattingConfig, ColumnSchemaConfig};
use customers::Customer;
use inventory::InventoryItem;

#[derive(Serialize)]
pub struct FrontendTableView(Vec<FrontendColumnView>);

#[derive(Serialize)]
pub struct FrontendColumnView {
    name: String,
    data_type: String,
    display_name: String,
    trimmable: bool,
    formatting: Option<ColumnFormattingConfig>,
}

static DB: OnceLock<Pool<Postgres>> = OnceLock::new();

#[macro_export]
macro_rules! get_db {
    () => {
        $crate::database::DB.get().unwrap()
    };
}

pub async fn connect() {
    let db = sqlx::postgres::PgPoolOptions::new()
        .max_connections(5)
        .connect("postgresql://techtriage:techtriage@localhost:5432")
        .await
        .unwrap();

    DB.get_or_init(|| db);

    let setup_script = config::create_setup_script();
    println!("{setup_script}");
    get_db!().batch_execute(&setup_script).await.unwrap();
}

pub fn get_frontend_view(table_name: &str) -> Json<FrontendTableView> {
    let config = CONFIG.get().unwrap();
    let table = config.tables.iter().find(|t| t.name == table_name).unwrap();
    let view = config.views.iter().find(|v| v.table == table_name).unwrap();

    // TODO: Redundant work - maybe store the full set of columns elsewhere for reuse
    let table_columns = iter::once(&table.primary_column)
        .chain(&table.required_columns)
        .chain(&table.optional_columns)
        .collect::<Vec<&ColumnSchemaConfig>>();

    let column_views = view
        .columns
        .iter()
        .map(|col| FrontendColumnView {
            name: col.name.clone(),
            data_type: table_columns
                .iter()
                .find(|c| c.name == col.name)
                .unwrap()
                .data_type
                .clone(),
            display_name: col.display_name.clone(),
            trimmable: col.trimmable,
            formatting: col.formatting.clone(),
        })
        .collect::<Vec<FrontendColumnView>>();

    Json(FrontendTableView(column_views))
}

pub async fn add_items(inventory_items: Vec<InventoryItem>, customers: Vec<Customer>) {
    const BIND_LIMIT: usize = u16::MAX as usize;
    const INVENTORY_ITEM_PARAMETERS: usize = 5;
    const CUSTOMER_PARAMETERS: usize = 5;
    const INVENTORY_CHUNK_SIZE: usize = BIND_LIMIT / INVENTORY_ITEM_PARAMETERS;
    const CUSTOMERS_CHUNK_SIZE: usize = BIND_LIMIT / CUSTOMER_PARAMETERS;

    let num_inventory_chunks = usize::div_ceil(inventory_items.len(), INVENTORY_CHUNK_SIZE);
    let num_customers_chunks = usize::div_ceil(customers.len(), CUSTOMERS_CHUNK_SIZE);
    let inventory_chunks = inventory_items.into_iter().chunks(num_inventory_chunks);
    let customers_chunks = customers.into_iter().chunks(num_customers_chunks);

    for chunk in &inventory_chunks {
        let mut inventory_insert_builder: QueryBuilder<Postgres> =
            QueryBuilder::new("INSERT INTO inventory (sku, display_name, count, cost, price) ");

        inventory_insert_builder.push_values(chunk, |mut b, item| {
            b.push_bind(item.sku)
                .push_bind(item.display_name)
                .push_bind(item.count)
                .push_bind(item.cost)
                .push_bind(item.price);
        });

        inventory_insert_builder
            .build()
            .execute(get_db!())
            .await
            .unwrap();
    }

    for chunk in &customers_chunks {
        let mut customers_insert_builder: QueryBuilder<Postgres> =
            QueryBuilder::new("INSERT INTO customers (id, name, email, phone, address) ");

        customers_insert_builder.push_values(chunk, |mut b, customer| {
            b.push_bind(customer.id)
                .push_bind(customer.name)
                .push_bind(customer.email)
                .push_bind(customer.phone)
                .push_bind(customer.address);
        });

        customers_insert_builder
            .build()
            .execute(get_db!())
            .await
            .unwrap();
    }
}
