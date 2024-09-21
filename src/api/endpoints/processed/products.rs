use rust_decimal::Decimal;
use serde::Serialize;

use proc_macros::{FromDatabaseEntity, ProcessEndpoint, ServeEntityJson, ServeRowJson};

use crate::api::endpoints::ViewCell;
use crate::api::{FromDatabaseRow, GenericIdParameter};
use crate::database::views::products::{ProductsDatabaseView, ProductsDatabaseViewRow};
use crate::database::DatabaseEntity;

#[derive(FromDatabaseEntity, ServeEntityJson, Serialize)]
#[database_entity(ProductsDatabaseView)]
pub struct ProductsApiEndpoint {
    metadata: EndpointMetadata,
    rows: Vec<ProductsApiEndpointRow>,
}

#[derive(ProcessEndpoint, ServeRowJson, Serialize)]
#[id_param(GenericIdParameter)]
pub struct ProductsApiEndpointRow {
    #[col_format(preset = "id", display_name = "SKU")]
    sku: ViewCell<u32>,
    #[col_format(preset = "string-notrim", display_name = "Name")]
    display_name: ViewCell<String>,
    #[col_format(preset = "currency")]
    cost: ViewCell<Decimal>,
    #[col_format(preset = "currency")]
    price: ViewCell<Decimal>,
}

impl FromDatabaseRow for ProductsApiEndpointRow {
    type Row = ProductsDatabaseViewRow;
    fn from_database_row(row: Self::Row) -> Self {
        let formatting = EndpointFormatting::new();

        let ProductsDatabaseViewRow {
            sku,
            display_name,
            cost,
            price,
        } = row;

        ProductsApiEndpointRow {
            sku: ViewCell::new(sku as u32, &formatting.sku),
            display_name: ViewCell::new(display_name, &formatting.display_name),
            cost: ViewCell::new(cost, &formatting.cost),
            price: ViewCell::new(price, &formatting.price),
        }
    }
}
