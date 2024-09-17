use rust_decimal::Decimal;
use serde::Serialize;

use proc_macros::{ServeEntityJson, ServeRowJson};

use crate::api::{FromDatabaseEntity, FromDatabaseRow, GenericIdParameter};
use crate::database::views::products::{ProductsDatabaseView, ProductsDatabaseViewRow};
use crate::database::DatabaseEntity;

#[derive(ServeEntityJson, Serialize)]
pub struct ProductsApiEndpoint {
    rows: Vec<ProductsApiEndpointRow>,
}

#[derive(ServeRowJson, Serialize)]
#[id_param(GenericIdParameter)]
pub struct ProductsApiEndpointRow {
    pub sku: i32,
    pub display_name: String,
    pub cost: Decimal,
    pub price: Decimal,
}

impl FromDatabaseEntity for ProductsApiEndpoint {
    type Entity = ProductsDatabaseView;
    fn from_database_entity(entity: Self::Entity) -> Self {
        Self {
            rows: entity
                .take_rows()
                .into_iter()
                .map(ProductsApiEndpointRow::from_database_row)
                .collect(),
        }
    }
}

impl FromDatabaseRow for ProductsApiEndpointRow {
    type Row = ProductsDatabaseViewRow;
    fn from_database_row(row: Self::Row) -> Self {
        ProductsApiEndpointRow {
            sku: row.sku,
            display_name: row.display_name,
            cost: row.cost,
            price: row.price,
        }
    }
}
