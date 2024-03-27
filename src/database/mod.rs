pub mod config;
pub mod customers;
pub mod inventory;

use std::iter;
use std::sync::OnceLock;

use axum::Json;
use serde::Serialize;
use tokio_postgres::{Client, Config, NoTls};

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

static DB: OnceLock<Client> = OnceLock::new();

#[macro_export]
macro_rules! get_db {
    () => {
        $crate::database::DB.get().unwrap()
    };
}

pub async fn connect() {
    let mut connection_config = Config::new();
    connection_config
        .user("techtriage")
        .password("techtriage")
        .host("localhost")
        .port(50589);

    let (client, connection) = connection_config.connect(NoTls).await.unwrap();

    DB.get_or_init(|| client);

    tokio::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("Connection error: {e}");
        }
    });

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

pub async fn add_items(inventory_items: &[InventoryItem], customers: &[Customer]) {
    for item in inventory_items {
        get_db!()
            .execute(&item.build_query(), &[&item.display_name])
            .await
            .unwrap();
    }

    for customer in customers {
        get_db!()
            .execute(
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
}
