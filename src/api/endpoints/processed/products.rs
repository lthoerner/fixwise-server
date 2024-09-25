use rust_decimal::Decimal;
use serde::Serialize;

use proc_macros::{FromRecord, FromRelation, ProcessEndpoint, ServeEntityJson, ServeRowJson};

use crate::api::endpoints::ViewCell;
use crate::api::GenericIdParameter;
use crate::database::views::products::{ProductsView, ProductsViewRecord};
use crate::database::Relation;

#[derive(FromRelation, ServeEntityJson, Serialize)]
#[endpoint(relation = ProductsView, raw = false)]
pub struct ProductsApiEndpoint {
    metadata: EndpointMetadata,
    rows: Vec<ProductsApiEndpointRow>,
}

#[derive(ProcessEndpoint, FromRecord, ServeRowJson, Serialize)]
#[endpoint_row(id_param = GenericIdParameter, record = ProductsViewRecord, raw = false)]
pub struct ProductsApiEndpointRow {
    #[col_format(preset = "id", display_name = "SKU")]
    sku: ViewCell<i32>,
    #[col_format(preset = "string-notrim", display_name = "Name")]
    display_name: ViewCell<String>,
    #[col_format(preset = "currency")]
    cost: ViewCell<Decimal>,
    #[col_format(preset = "currency")]
    price: ViewCell<Decimal>,
}