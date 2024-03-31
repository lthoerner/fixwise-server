use serde::{Deserialize, Serialize};
use sqlx::raw_sql;

#[derive(Serialize, Deserialize)]
pub struct FrontendTableView {
    name: String,
    columns: Vec<FrontendColumnView>,
}

#[derive(Serialize, Deserialize)]
pub struct FrontendColumnView {
    name: String,
    display_name: String,
    data_type: String,
    trimmable: bool,
    formatting: Option<ColumnFormatting>,
}

#[derive(Serialize, Deserialize)]
pub struct ColumnFormatting {
    prefix: Option<String>,
    suffix: Option<String>,
    pad_length: Option<u64>,
}

pub(super) async fn configure_db() {
    const CONFIG_SCRIPT: &str = include_str!("../../database/config.sql");

    raw_sql(CONFIG_SCRIPT)
        .execute(crate::get_db!())
        .await
        .unwrap();
}
