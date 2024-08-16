use std::sync::Arc;

use axum::extract::{Json, Query, State};
use serde::Serialize;

use crate::api::{FromDatabaseRow, ServeRowJson};
use crate::database::tables::type_allocation_codes::{
    TypeAllocationCodesDatabaseTable, TypeAllocationCodesDatabaseTableRow,
};
use crate::database::{DatabaseEntity, IdParameter};
use crate::ServerState;

#[derive(Serialize)]
pub struct ImeiInfoApiUtil {
    manufacturer: String,
    model: String,
}

impl ServeRowJson for ImeiInfoApiUtil {
    async fn serve_one(
        state: State<Arc<ServerState>>,
        id_param: Query<IdParameter>,
    ) -> Json<Option<Self>> {
        if let Some(existing_row) = Self::Entity::query_one(state.clone(), id_param.clone()).await {
            Json(Some(Self::from_database_row(existing_row)))
        } else {
            let backend_imei_info =
                imei_info::get_imei_info(&state.imei_info_api_key, &id_param.0.id.to_string())
                    .await
                    .unwrap();
            let frontend_imei_info = ImeiInfoApiUtil {
                manufacturer: backend_imei_info.manufacturer,
                model: backend_imei_info.model,
            };

            Json(Some(frontend_imei_info))
        }
    }
}

impl FromDatabaseRow for ImeiInfoApiUtil {
    type Row = TypeAllocationCodesDatabaseTableRow;
    type Entity = TypeAllocationCodesDatabaseTable;
    fn from_database_row(row: Self::Row) -> Self {
        ImeiInfoApiUtil {
            manufacturer: row.manufacturer,
            model: row.model,
        }
    }
}
