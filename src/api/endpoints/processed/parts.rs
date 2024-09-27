use rust_decimal::Decimal;
use serde::Serialize;

use proc_macros::{FromRecord, FromRelation, ProcessEndpoint, ServeRecordJson, ServeResourceJson};

use crate::api::endpoints::ViewCell;
use crate::api::GenericIdParameter;
use crate::database::views::parts::{PartsView, PartsViewRecord};
use crate::database::Relation;

#[derive(FromRelation, ServeResourceJson, Serialize)]
#[resource(relation = PartsView, raw = false)]
pub struct PartsResource {
    metadata: EndpointMetadata,
    records: Vec<PartsResourceRecord>,
}

#[derive(ProcessEndpoint, FromRecord, ServeRecordJson, Serialize)]
#[resource_record(id_param = GenericIdParameter, record = PartsViewRecord, raw = false)]
pub struct PartsResourceRecord {
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
