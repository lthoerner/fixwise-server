pub mod shared_models;
pub mod tables;
pub mod views;

use std::sync::Arc;

use axum::extract::State;
use itertools::Itertools;
use sqlx::postgres::PgRow;
use sqlx::{raw_sql, PgPool, Postgres, QueryBuilder};

use crate::ServerState;
use tables::customers::CustomersDatabaseTableRow;
use tables::inventory::InventoryDatabaseTableRow;
use tables::tickets::TicketsDatabaseTableRow;

#[derive(Clone)]
pub struct Database {
    pub connection: PgPool,
}

pub trait DatabaseEntity: Sized {
    type Row: for<'a> sqlx::FromRow<'a, PgRow> + Send + Unpin;

    const ENTITY_NAME: &'static str;
    const PRIMARY_COLUMN_NAME: &'static str;

    fn with_rows(rows: Vec<Self::Row>) -> Self;
    fn rows(self) -> Vec<Self::Row>;

    async fn query_all(State(state): State<Arc<ServerState>>) -> Self {
        Self::with_rows(
            sqlx::query_as(&format!(
                "SELECT * FROM main.{} ORDER BY {}",
                Self::ENTITY_NAME,
                Self::PRIMARY_COLUMN_NAME
            ))
            .fetch_all(&state.database.connection)
            .await
            .unwrap(),
        )
    }
}

impl Database {
    const CONFIG_SCRIPT: &'static str = include_str!("../../database/config.sql");

    pub async fn connect_and_configure() -> Self {
        let database = Self::connect().await;
        database.configure().await;

        database
    }

    async fn connect() -> Self {
        Self {
            connection: PgPool::connect("postgresql://techtriage:techtriage@localhost:5432")
                .await
                .unwrap(),
        }
    }

    async fn configure(&self) {
        raw_sql(Self::CONFIG_SCRIPT)
            .execute(&self.connection)
            .await
            .unwrap();
    }

    pub async fn add_items(
        &self,
        inventory_items: Vec<InventoryDatabaseTableRow>,
        customers: Vec<CustomersDatabaseTableRow>,
        tickets: Vec<TicketsDatabaseTableRow>,
    ) {
        const BIND_LIMIT: usize = u16::MAX as usize;
        const INVENTORY_ITEM_PARAMETERS: usize = 5;
        const CUSTOMER_PARAMETERS: usize = 5;
        const TICKET_PARAMETERS: usize = 8;
        const INVENTORY_CHUNK_SIZE: usize = BIND_LIMIT / INVENTORY_ITEM_PARAMETERS;
        const CUSTOMERS_CHUNK_SIZE: usize = BIND_LIMIT / CUSTOMER_PARAMETERS;
        const TICKETS_CHUNK_SIZE: usize = BIND_LIMIT / TICKET_PARAMETERS;

        let num_inventory_chunks = usize::div_ceil(inventory_items.len(), INVENTORY_CHUNK_SIZE);
        let num_customers_chunks = usize::div_ceil(customers.len(), CUSTOMERS_CHUNK_SIZE);
        let num_tickets_chunks = usize::div_ceil(tickets.len(), TICKETS_CHUNK_SIZE);
        let inventory_chunks = inventory_items.into_iter().chunks(num_inventory_chunks);
        let customers_chunks = customers.into_iter().chunks(num_customers_chunks);
        let tickets_chunks = tickets.into_iter().chunks(num_tickets_chunks);

        for chunk in &inventory_chunks {
            // TODO: Generate this query from a const list of fields or something
            let mut inventory_insert_builder: QueryBuilder<Postgres> =
                QueryBuilder::new("INSERT INTO main.inventory (sku, name, count, cost, price) ");

            inventory_insert_builder.push_values(chunk, |mut b, item| {
                b.push_bind(item.sku)
                    .push_bind(item.name)
                    .push_bind(item.count)
                    .push_bind(item.cost)
                    .push_bind(item.price);
            });

            inventory_insert_builder
                .build()
                .execute(&self.connection)
                .await
                .unwrap();
        }

        for chunk in &customers_chunks {
            let mut customers_insert_builder: QueryBuilder<Postgres> =
                QueryBuilder::new("INSERT INTO main.customers (id, name, email, phone, address) ");

            customers_insert_builder.push_values(chunk, |mut b, customer| {
                b.push_bind(customer.id)
                    .push_bind(customer.name)
                    .push_bind(customer.email)
                    .push_bind(customer.phone)
                    .push_bind(customer.address);
            });

            customers_insert_builder
                .build()
                .execute(&self.connection)
                .await
                .unwrap();
        }

        for chunk in &tickets_chunks {
            let mut tickets_insert_builder: QueryBuilder<Postgres> = QueryBuilder::new(
                "INSERT INTO main.tickets (id, status, customer_id, device, diagnostic, invoice_amount, payment_amount, created_at, updated_at) ",
            );

            tickets_insert_builder.push_values(chunk, |mut b, ticket| {
                b.push_bind(ticket.id)
                    .push_bind(ticket.status)
                    .push_bind(ticket.customer_id)
                    .push_bind(ticket.device)
                    .push_bind(ticket.diagnostic)
                    .push_bind(ticket.invoice_amount)
                    .push_bind(ticket.payment_amount)
                    .push_bind(ticket.created_at)
                    .push_bind(ticket.updated_at);
            });

            tickets_insert_builder
                .build()
                .execute(&self.connection)
                .await
                .unwrap();
        }
    }
}
