use axum::{extract::State, Json};
use serde::{Deserialize, Serialize};

use crate::ServerState;

#[derive(Clone)]
pub struct ViewConfigurations {
    pub(super) inventory: ViewConfiguration,
    pub(super) customers: ViewConfiguration,
    pub(super) tickets: ViewConfiguration,
}

#[derive(Clone)]
pub(super) struct ViewConfiguration {
    pub(super) backend: BackendViewConfiguration,
    frontend: FrontendViewConfiguration,
}

#[derive(Clone, Serialize)]
pub struct FrontendViewConfiguration {
    columns: Vec<FrontendColumnViewConfiguration>,
}

#[derive(Clone, Serialize)]
struct FrontendColumnViewConfiguration {
    name: String,
    data_type: String,
    display_name: String,
    trimmable: bool,
}

#[derive(Clone, Deserialize)]
pub(super) struct BackendViewConfiguration {
    pub(super) columns: Vec<BackendColumnViewConfiguration>,
}

#[derive(Clone, Deserialize)]
pub(super) struct BackendColumnViewConfiguration {
    pub(super) view_key: String,
    data_type: String,
    display: ColumnDisplay,
    pub(super) formatting: Option<ColumnFormatting>,
}

#[derive(Clone, Serialize, Deserialize)]
struct ColumnDisplay {
    name: String,
    trimmable: bool,
}

#[derive(Clone, Deserialize)]
pub(super) struct ColumnFormatting {
    pub(super) prefix: Option<String>,
    pub(super) suffix: Option<String>,
    pub(super) pad_length: Option<usize>,
}

pub async fn get_inventory_view(
    State(state): State<ServerState>,
) -> Json<FrontendViewConfiguration> {
    Json(state.view_configurations.inventory.frontend)
}

pub async fn get_customers_view(
    State(state): State<ServerState>,
) -> Json<FrontendViewConfiguration> {
    Json(state.view_configurations.customers.frontend)
}

pub async fn get_tickets_view(State(state): State<ServerState>) -> Json<FrontendViewConfiguration> {
    Json(state.view_configurations.tickets.frontend)
}

impl ViewConfigurations {
    const INVENTORY_CONFIGURATION_STR: &'static str = include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/database/view_configurations/inventory.toml"
    ));
    const CUSTOMERS_CONFIGURATION_STR: &'static str = include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/database/view_configurations/customers.toml"
    ));
    const TICKETS_CONFIGURATION_STR: &'static str = include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/database/view_configurations/tickets.toml"
    ));

    pub fn load() -> Self {
        Self {
            inventory: ViewConfiguration::from(Self::INVENTORY_CONFIGURATION_STR),
            customers: ViewConfiguration::from(Self::CUSTOMERS_CONFIGURATION_STR),
            tickets: ViewConfiguration::from(Self::TICKETS_CONFIGURATION_STR),
        }
    }
}

impl BackendViewConfiguration {
    pub(super) fn get_column_formatting(&self, column: &str) -> Option<&ColumnFormatting> {
        if let Some(column) = self.columns.iter().find(|col| col.view_key == column) {
            column.formatting.as_ref()
        } else {
            None
        }
    }
}

impl From<&str> for ViewConfiguration {
    fn from(value: &str) -> Self {
        let backend = toml::from_str(value).unwrap();
        let frontend = FrontendViewConfiguration::from(&backend);

        Self { backend, frontend }
    }
}

impl From<&BackendViewConfiguration> for FrontendViewConfiguration {
    fn from(value: &BackendViewConfiguration) -> Self {
        FrontendViewConfiguration {
            columns: value.columns.iter().map(Into::into).collect(),
        }
    }
}

impl From<&BackendColumnViewConfiguration> for FrontendColumnViewConfiguration {
    fn from(value: &BackendColumnViewConfiguration) -> Self {
        FrontendColumnViewConfiguration {
            name: value.view_key.clone(),
            data_type: value.data_type.clone(),
            display_name: value.display.name.clone(),
            trimmable: value.display.trimmable,
        }
    }
}
