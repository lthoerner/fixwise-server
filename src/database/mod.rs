pub mod config;
pub mod customers;
pub mod inventory;

use std::iter;

use axum::Json;
use serde::Serialize;

use config::CONFIG;
use config::{ColumnFormattingConfig, ColumnSchemaConfig};

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
