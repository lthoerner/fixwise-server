pub mod utils;
pub mod views;

use std::sync::Arc;

use axum::extract::{Query, State};
use axum::Json;
use serde::Serialize;

use crate::database::{DatabaseEntity, IdParameter};
use crate::ServerState;

pub trait ServeEntityJson: FromDatabaseEntity + Serialize + Sized {
    async fn serve_all(state: State<Arc<ServerState>>) -> Json<Self> {
        Json(Self::from_database_entity(
            <Self as FromDatabaseEntity>::Entity::query_all(state).await,
        ))
    }
}

pub trait ServeRowJson<I: IdParameter>: FromDatabaseRow + Serialize + Sized {
    async fn serve_one(state: State<Arc<ServerState>>, id_param: Query<I>) -> Json<Option<Self>> {
        Json(Some(Self::from_database_row(
            <<Self as FromDatabaseRow>::Entity as DatabaseEntity>::query_one(state, id_param)
                .await
                .unwrap(),
        )))
    }
}

pub trait FromDatabaseEntity {
    type Entity: DatabaseEntity;
    fn from_database_entity(entity: Self::Entity) -> Self;
}

pub trait FromDatabaseRow {
    type Row;
    type Entity: DatabaseEntity<Row = Self::Row>;
    fn from_database_row(row: Self::Row) -> Self;
}
