use std::sync::Arc;

use axum::extract::{Json, Query, State};
use imei_info::PhoneInfo;
use serde::Serialize;

use crate::api::{FromDatabaseRow, ServeRowJson};
use crate::database::tables::type_allocation_codes::{
    TypeAllocationCodesDatabaseTable, TypeAllocationCodesDatabaseTableRow,
};
use crate::database::{BulkInsert, DatabaseEntity, GenericIdParameter};
use crate::ServerState;

#[derive(Serialize)]
pub struct ImeiInfoApiUtil {
    manufacturer: String,
    model: String,
}

impl ServeRowJson for ImeiInfoApiUtil {
    async fn serve_one(
        state: State<Arc<ServerState>>,
        imei_param: Query<GenericIdParameter>,
    ) -> Json<Option<Self>> {
        if let Some(existing_row) = Self::Entity::query_one(state.clone(), imei_param.clone()).await
        {
            Json(Some(Self::from_database_row(existing_row)))
        } else {
            let imei_string = format!("{:08}", &imei_param.0.id);
            let PhoneInfo {
                imei,
                manufacturer,
                model,
            } = imei_info::get_imei_info(&state.imei_info_api_key, &imei_string)
                .await
                .unwrap();

            let database_imei_info = TypeAllocationCodesDatabaseTableRow {
                tac: imei
                    .type_allocation_code()
                    .iter()
                    .enumerate()
                    .map(|(i, d)| *d as i32 * 10 ^ i as i32)
                    .sum(),
                manufacturer: manufacturer.clone(),
                model: model.clone(),
            };

            TypeAllocationCodesDatabaseTable::with_rows(Vec::from([database_imei_info]))
                .insert_all(&state.database)
                .await;

            let frontend_imei_info = ImeiInfoApiUtil {
                manufacturer,
                model,
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
