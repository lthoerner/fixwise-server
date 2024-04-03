pub mod views;

use axum::extract::State;
use axum::Json;
use serde::Serialize;

use crate::database::DatabaseEntity;
use crate::ServerState;

pub trait ServeJson: Sized {
    async fn serve_json(state: State<ServerState>) -> Json<Self>;
}

trait FromDatabaseEntity {
    type Entity: DatabaseEntity;
    fn from_database_entity(entity: Self::Entity) -> Self;
}

impl<T: FromDatabaseEntity + Serialize> ServeJson for T {
    async fn serve_json(state: State<ServerState>) -> Json<T> {
        Json(T::from_database_entity(T::Entity::query_all(state).await))
    }
}
