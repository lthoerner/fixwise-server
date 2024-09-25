use rust_decimal::Decimal;
use serde::Serialize;

use proc_macros::{FromRecord, FromRelation, ProcessEndpoint, ServeEntityJson, ServeRowJson};

use crate::api::endpoints::ViewCell;
use crate::api::GenericIdParameter;
use crate::database::views::parts::{PartsView, PartsViewRecord};
use crate::database::Relation;

#[derive(FromRelation, ServeEntityJson, Serialize)]
#[endpoint(relation = PartsView, raw = false)]
pub struct PartsApiEndpoint {
    metadata: EndpointMetadata,
    rows: Vec<PartsApiEndpointRow>,
}

#[derive(ProcessEndpoint, FromRecord, ServeRowJson, Serialize)]
#[endpoint_row(id_param = GenericIdParameter, record = PartsViewRecord, raw = false)]
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
