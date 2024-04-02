pub mod views;

use axum::extract::State;
use axum::Json;
use serde::Serialize;

use crate::database::DatabaseEntity;
use crate::ServerState;

pub trait ServeJson: Sized {
    async fn serve_json(state: State<ServerState>) -> Json<Vec<Self>>;
}

trait FromDatabaseRow {
    type Entity: DatabaseEntity;
    fn from_database_row(row: Self::Entity) -> Self;
}

impl<T: FromDatabaseRow + Serialize> ServeJson for T {
    async fn serve_json(state: State<ServerState>) -> Json<Vec<Self>> {
        Json(
            T::Entity::query_all(state)
                .await
                .into_iter()
                .map(FromDatabaseRow::from_database_row)
                .collect(),
        )
    }
}
