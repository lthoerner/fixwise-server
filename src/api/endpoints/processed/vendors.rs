use serde::Serialize;

use proc_macros::{
    FromDatabaseEntity, FromDatabaseRow, ProcessEndpoint, ServeEntityJson, ServeRowJson,
};

use crate::api::endpoints::ViewCell;
use crate::api::GenericIdParameter;
use crate::database::views::vendors::{VendorsDatabaseView, VendorsDatabaseViewRow};
use crate::database::DatabaseEntity;

#[derive(FromDatabaseEntity, ServeEntityJson, Serialize)]
#[endpoint(database_entity = VendorsDatabaseView, raw = false)]
pub struct VendorsApiEndpoint {
    metadata: EndpointMetadata,
    rows: Vec<VendorsApiEndpointRow>,
}

#[derive(ProcessEndpoint, FromDatabaseRow, ServeRowJson, Serialize)]
#[endpoint_row(id_param = GenericIdParameter, database_row = VendorsDatabaseViewRow, raw = false)]
pub struct VendorsApiEndpointRow {
    #[col_format(preset = "id")]
    id: ViewCell<i32>,
    #[col_format(preset = "string-notrim", display_name = "Name")]
    display_name: ViewCell<String>,
}
