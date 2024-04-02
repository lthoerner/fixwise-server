pub mod api;
pub mod customers;
pub mod inventory;
pub mod tickets;
pub mod views;

use itertools::Itertools;
use sqlx::{raw_sql, PgPool, Postgres, QueryBuilder};

use customers::Customer;
use inventory::InventoryItem;
use tickets::Ticket;

#[derive(Clone)]
pub struct Database {
    pub connection: PgPool,
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
        inventory_items: Vec<InventoryItem>,
        customers: Vec<Customer>,
        tickets: Vec<Ticket>,
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
            let mut inventory_insert_builder: QueryBuilder<Postgres> = QueryBuilder::new(
                "INSERT INTO test.inventory (sku, display_name, count, cost, price) ",
            );

            inventory_insert_builder.push_values(chunk, |mut b, item| {
                b.push_bind(item.sku.base)
                    .push_bind(item.display_name.base)
                    .push_bind(item.count.base)
                    .push_bind(item.cost.base)
                    .push_bind(item.price.base);
            });

            inventory_insert_builder
                .build()
                .execute(&self.connection)
                .await
                .unwrap();
        }

        for chunk in &customers_chunks {
            let mut customers_insert_builder: QueryBuilder<Postgres> =
                QueryBuilder::new("INSERT INTO test.customers (id, name, email, phone, address) ");

            customers_insert_builder.push_values(chunk, |mut b, customer| {
                b.push_bind(customer.id.base)
                    .push_bind(customer.name.base)
                    .push_bind(customer.email.base)
                    .push_bind(customer.phone.base)
                    .push_bind(customer.address.base);
            });

            customers_insert_builder
                .build()
                .execute(&self.connection)
                .await
                .unwrap();
        }

        for chunk in &tickets_chunks {
            let mut tickets_insert_builder: QueryBuilder<Postgres> = QueryBuilder::new(
                "INSERT INTO test.tickets (id, customer, device, diagnostic, invoice_amount, payment_amount, created_at, updated_at) ",
            );

            tickets_insert_builder.push_values(chunk, |mut b, ticket| {
                b.push_bind(ticket.id)
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
