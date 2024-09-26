use rust_decimal::Decimal;
use serde::Serialize;

use proc_macros::{FromRecord, FromRelation, ProcessEndpoint, ServeResourceJson, ServeRecordJson};

use crate::api::endpoints::ViewCell;
use crate::api::GenericIdParameter;
use crate::database::views::products::{ProductsView, ProductsViewRecord};
use crate::database::Relation;

#[derive(FromRelation, ServeResourceJson, Serialize)]
#[resource(relation = ProductsView, raw = false)]
pub struct ProductsResource {
    metadata: EndpointMetadata,
    records: Vec<ProductsResourceRecord>,
}

#[derive(ProcessEndpoint, FromRecord, ServeRecordJson, Serialize)]
#[resource_record(id_param = GenericIdParameter, record = ProductsViewRecord, raw = false)]
pub struct ProductsResourceRecord {
    #[col_format(preset = "id", display_name = "SKU")]
    sku: ViewCell<i32>,
    #[col_format(preset = "string-notrim", display_name = "Name")]
    display_name: ViewCell<String>,
    #[col_format(preset = "currency")]
    cost: ViewCell<Decimal>,
    #[col_format(preset = "currency")]
    price: ViewCell<Decimal>,
}
