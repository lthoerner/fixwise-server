use std::sync::Arc;

use axum::extract::{Json, Query, State};
use crudkit::database::DatabaseState;
use crudkit::traits::read::{ReadRecord, ReadRelation};
use crudkit::traits::write::SingleInsert;
use crudkit::IdParameter;
use imei_info::{Imei, PhoneInfo, Tac};
use serde::{Deserialize, Serialize};

use proc_macros::FromRecord;

use crate::api::{IdParameter, ServeRecordJson};
use crate::database::tables::type_allocation_codes::TypeAllocationCodesTableRecord;
use crate::ServerState;

#[derive(Clone, Deserialize, IdParameter)]
pub struct ImeiParameter {
    imei: usize,
}

#[derive(FromRecord, Serialize)]
#[resource_record(id_param = GenericIdParameter, record = TypeAllocationCodesTableRecord, raw = true)]
pub struct ImeiInfoApiUtil {
    manufacturer: String,
    model: String,
}

impl ServeRecordJson<ImeiParameter> for ImeiInfoApiUtil {
    async fn serve_one(
        state: State<Arc<ServerState>>,
        imei_param: Query<ImeiParameter>,
    ) -> Json<Option<Self>> {
        let imei = Imei::try_from(imei_param.0.id()).unwrap();
        let tac = Tac::from(imei.clone());
        if let Ok(existing_row) = <Self::Record as ReadRecord>::ReadRelation::query_one(
            state.get_database(),
            ImeiParameter::new(tac.clone().into()),
        )
        .await
        {
            Json(Some(<Self as crate::api::FromRecord>::from_record(
                existing_row,
            )))
        } else {
            let PhoneInfo {
                manufacturer,
                model,
                ..
            } = imei_info::get_imei_info(&state.imei_info_api_key, imei)
                .await
                .unwrap();

            let database_imei_info = TypeAllocationCodesTableRecord {
                tac: tac.into(),
                manufacturer: manufacturer.clone(),
                model: model.clone(),
            };

            database_imei_info
                .insert(state.get_database())
                .await
                .unwrap();

            let frontend_imei_info = ImeiInfoApiUtil {
                manufacturer,
                model,
            };

            Json(Some(frontend_imei_info))
        }
    }
}
