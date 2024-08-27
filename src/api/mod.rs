pub mod utils;
pub mod views;

use std::sync::Arc;

use axum::extract::{Query, State};
use axum::Json;
use serde::{Deserialize, Serialize};

use proc_macros::IdParameter;

use crate::database::{DatabaseEntity, DatabaseRow};
use crate::ServerState;

pub trait ServeEntityJson: FromDatabaseEntity + Serialize + Sized {
    async fn serve_all(state: State<Arc<ServerState>>) -> Json<Self> {
        Json(Self::from_database_entity(
            Self::Entity::query_all(state).await,
        ))
    }
}

pub trait ServeRowJson<I: IdParameter>: FromDatabaseRow + Serialize + Sized {
    async fn serve_one(state: State<Arc<ServerState>>, id_param: Query<I>) -> Json<Option<Self>> {
        Json(Some(Self::from_database_row(
            Self::Row::query_one(state, id_param).await.unwrap(),
        )))
    }
}

pub trait FromDatabaseEntity {
    type Entity: DatabaseEntity;
    fn from_database_entity(entity: Self::Entity) -> Self;
}

pub trait FromDatabaseRow {
    type Row: DatabaseRow;
    fn from_database_row(row: Self::Row) -> Self;
}

pub trait IdParameter {
    fn new(value: usize) -> Self;
    fn id(&self) -> usize;
}

#[derive(Clone, Deserialize, IdParameter)]
pub struct GenericIdParameter {
    id: usize,
}
