pub mod config;
pub mod customers;
pub mod inventory;

use std::sync::OnceLock;

use itertools::Itertools;
use sqlx::{PgPool, Postgres, QueryBuilder};

use customers::Customer;
use inventory::InventoryItem;

static DB: OnceLock<PgPool> = OnceLock::new();

#[macro_export]
macro_rules! get_db {
    () => {
        $crate::database::DB.get().unwrap()
    };
}

pub async fn connect() {
    let db = PgPool::connect("postgresql://techtriage:techtriage@localhost:5432")
        .await
        .unwrap();

    DB.get_or_init(|| db);

    config::configure_db().await;
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
        let mut inventory_insert_builder: QueryBuilder<Postgres> = QueryBuilder::new(
            "INSERT INTO test.inventory (sku, display_name, count, cost, price) ",
        );

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
            QueryBuilder::new("INSERT INTO test.customers (id, name, email, phone, address) ");

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
