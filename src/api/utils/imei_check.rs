use std::sync::Arc;

use axum::extract::{Json, Query, State};
use imei_info::{Imei, PhoneInfo, Tac};
use serde::{Deserialize, Serialize};

use crate::api::{FromDatabaseRow, IdParameter, ServeRowJson};
use crate::database::tables::type_allocation_codes::TypeAllocationCodesDatabaseTableRow;
use crate::database::{DatabaseRow, SingleInsert};
use crate::ServerState;

#[derive(Clone, Deserialize, IdParameter)]
pub struct ImeiParameter {
    imei: usize,
}

#[derive(Serialize)]
pub struct ImeiInfoApiUtil {
    manufacturer: String,
    model: String,
}

impl ServeRowJson<ImeiParameter> for ImeiInfoApiUtil {
    async fn serve_one(
        state: State<Arc<ServerState>>,
        imei_param: Query<ImeiParameter>,
    ) -> Json<Option<Self>> {
        let imei = Imei::try_from(imei_param.0.id()).unwrap();
        let tac = Tac::from(imei.clone());
        if let Some(existing_row) =
            Self::Row::query_one(state.clone(), Query(ImeiParameter::new(tac.clone().into()))).await
        {
            Json(Some(Self::from_database_row(existing_row)))
        } else {
            let PhoneInfo {
                manufacturer,
                model,
                ..
            } = imei_info::get_imei_info(&state.imei_info_api_key, imei)
                .await
                .unwrap();

            let database_imei_info = TypeAllocationCodesDatabaseTableRow {
                tac: tac.into(),
                manufacturer: manufacturer.clone(),
                model: model.clone(),
            };

            database_imei_info.insert(&state.database).await;

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
    fn from_database_row(row: Self::Row) -> Self {
        ImeiInfoApiUtil {
            manufacturer: row.manufacturer,
            model: row.model,
        }
    }
}
