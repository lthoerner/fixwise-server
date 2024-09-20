use serde::Serialize;

use proc_macros::{FromDatabaseEntity, ProcessEndpoint, ServeEntityJson, ServeRowJson};

use crate::api::endpoints::ViewCell;
use crate::api::{FromDatabaseRow, GenericIdParameter};
use crate::database::views::vendors::{VendorsDatabaseView, VendorsDatabaseViewRow};
use crate::database::DatabaseEntity;

#[derive(FromDatabaseEntity, ServeEntityJson, Serialize)]
#[database_entity(VendorsDatabaseView)]
pub struct VendorsApiEndpoint {
    metadata: EndpointMetadata,
    rows: Vec<VendorsApiEndpointRow>,
}

#[derive(ProcessEndpoint, ServeRowJson, Serialize)]
#[id_param(GenericIdParameter)]
pub struct VendorsApiEndpointRow {
    #[col_format(preset = "id")]
    id: ViewCell<u32>,
    #[col_format(preset = "string-notrim", display_name = "Name")]
    display_name: ViewCell<String>,
}

impl FromDatabaseRow for VendorsApiEndpointRow {
    type Row = VendorsDatabaseViewRow;
    fn from_database_row(row: Self::Row) -> Self {
        let formatting = EndpointFormatting::new();
        let VendorsDatabaseViewRow { id, display_name } = row;
        VendorsApiEndpointRow {
            id: ViewCell::new(id as u32, &formatting.id),
            display_name: ViewCell::new(display_name, &formatting.display_name),
        }
    }
}
