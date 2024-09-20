use rust_decimal::Decimal;
use serde::Serialize;

use proc_macros::{FromDatabaseEntity, ProcessEndpoint, ServeEntityJson, ServeRowJson};

use crate::api::endpoints::ViewCell;
use crate::api::{FromDatabaseRow, GenericIdParameter};
use crate::database::views::parts::{PartsDatabaseView, PartsDatabaseViewRow};
use crate::database::DatabaseEntity;

#[derive(FromDatabaseEntity, ServeEntityJson, Serialize)]
#[database_entity(PartsDatabaseView)]
pub struct PartsApiEndpoint {
    metadata: EndpointMetadata,
    rows: Vec<PartsApiEndpointRow>,
}

#[derive(ProcessEndpoint, ServeRowJson, Serialize)]
#[id_param(GenericIdParameter)]
pub struct PartsApiEndpointRow {
    #[col_format(
        format = "id",
        data_type = "integer",
        display_name = "ID",
        trimmable = false
    )]
    id: ViewCell<u32>,
    #[col_format(data_type = "string", display_name = "Name", trimmable = false)]
    display_name: ViewCell<String>,
    #[col_format(data_type = "string", trimmable = true)]
    vendor: ViewCell<String>,
    #[col_format(data_type = "string", trimmable = true)]
    manufacturer: ViewCell<Option<String>>,
    #[col_format(data_type = "string", trimmable = true)]
    category: ViewCell<String>,
    #[col_format(format = "currency", data_type = "decimal", trimmable = false)]
    cost: ViewCell<Option<Decimal>>,
    #[col_format(format = "currency", data_type = "decimal", trimmable = false)]
    price: ViewCell<Option<Decimal>>,
}

impl FromDatabaseRow for PartsApiEndpointRow {
    type Row = PartsDatabaseViewRow;
    fn from_database_row(row: Self::Row) -> Self {
        let formatting = EndpointFormatting::new();

        let PartsDatabaseViewRow {
            id,
            display_name,
            vendor,
            manufacturer,
            category,
            cost,
            price,
        } = row;

        PartsApiEndpointRow {
            id: ViewCell::new(id as u32, &formatting.id),
            display_name: ViewCell::new(display_name, &formatting.display_name),
            vendor: ViewCell::new(vendor, &formatting.vendor),
            manufacturer: ViewCell::new(manufacturer, &formatting.manufacturer),
            category: ViewCell::new(category, &formatting.category),
            cost: ViewCell::new(cost, &formatting.cost),
            price: ViewCell::new(price, &formatting.price),
        }
    }
}
