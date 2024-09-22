use rust_decimal::Decimal;
use serde::Serialize;

use proc_macros::{
    FromDatabaseEntity, FromDatabaseRow, ProcessEndpoint, ServeEntityJson, ServeRowJson,
};

use crate::api::endpoints::ViewCell;
use crate::api::GenericIdParameter;
use crate::database::views::parts::{PartsDatabaseView, PartsDatabaseViewRow};
use crate::database::DatabaseEntity;

#[derive(FromDatabaseEntity, ServeEntityJson, Serialize)]
#[database_entity(PartsDatabaseView)]
pub struct PartsApiEndpoint {
    metadata: EndpointMetadata,
    rows: Vec<PartsApiEndpointRow>,
}

#[derive(ProcessEndpoint, FromDatabaseRow, ServeRowJson, Serialize)]
#[endpoint_row(id_param = GenericIdParameter, database_row = PartsDatabaseViewRow)]
pub struct PartsApiEndpointRow {
    #[col_format(preset = "id")]
    id: ViewCell<i32>,
    #[col_format(preset = "string-notrim", display_name = "Name")]
    display_name: ViewCell<String>,
    #[col_format(preset = "string")]
    vendor: ViewCell<String>,
    #[col_format(preset = "string")]
    manufacturer: ViewCell<Option<String>>,
    #[col_format(preset = "string")]
    category: ViewCell<String>,
    #[col_format(preset = "currency")]
    cost: ViewCell<Option<Decimal>>,
    #[col_format(preset = "currency")]
    price: ViewCell<Option<Decimal>>,
}
