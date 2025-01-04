use std::sync::Arc;

use axum::extract::{Json, Query, State};

use super::shared::{Record, Relation};
use crate::api::IdParameter;
use crate::database::Database;
use crate::ServerState;

pub trait ReadRelation: Relation {
    type ReadRecord: ReadRecord<ReadRelation = Self>;

    /// Query (select) a single record from the database using an identifying key.
    ///
    /// If the record exists in the database, it is returned. Otherwise, [`None`] is returned.
    ///
    /// This is the standard version of this method and should not be used as an Axum route handler.
    /// For the handler method, use [`Relation::query_one_handler()`].
    async fn query_one<I: IdParameter>(database: &Database, id: I) -> Option<Self::ReadRecord> {
        sqlx::query_as(&format!(
            "SELECT * FROM {}.{} WHERE {} = #1",
            Self::SCHEMA_NAME,
            Self::RELATION_NAME,
            Self::PRIMARY_KEY,
        ))
        .bind(id.id() as i32)
        .fetch_one(&database.connection)
        .await
        .ok()
    }

    /// Query (select) a single record from the database using an identifying key.
    ///
    /// If the record exists in the database, it is returned. Otherwise, [`None`] is returned.
    ///
    /// This is the Axum route handler version of this method. For the standard method, which can be
    /// called outside of an Axum context, see [`Relation::query_one()`].
    // TODO: Check how this interacts with junction tables
    async fn query_one_handler<I: IdParameter>(
        state: State<Arc<ServerState>>,
        Query(id_param): Query<I>,
    ) -> Json<Option<Self::ReadRecord>> {
        Json(Self::query_one(&state.database, id_param).await)
    }

    /// Query (select) all records for this relation from the database.
    ///
    /// This is the standard version of this method and should not be used as an Axum route handler.
    /// For the handler method, use [`Relation::query_all_handler()`].
    async fn query_all(database: &Database) -> Self {
        Self::with_records(
            sqlx::query_as(&format!(
                "SELECT * FROM {}.{} ORDER BY {}",
                Self::SCHEMA_NAME,
                Self::RELATION_NAME,
                Self::PRIMARY_KEY,
            ))
            .fetch_all(&database.connection)
            .await
            .unwrap(),
        )
    }

    /// Query (select) all records for this relation from the database.
    ///
    /// This is the Axum route handler version of this method. For the standard method, which can be
    /// called outside of an Axum context, see [`Relation::query_all()`].
    async fn query_all_handler(state: State<Arc<ServerState>>) -> Json<Self> {
        Json(Self::query_all(&state.database).await)
    }
}

pub trait ReadRecord: Record {
    type ReadRelation: ReadRelation<ReadRecord = Self>;

    #[allow(dead_code)]
    /// Query (select) a single record from the database using an identifying key.
    ///
    /// If the record exists in the database, it is returned. Otherwise, [`None`] is returned.
    ///
    /// This is the standard version of this method and should not be used as an Axum route handler.
    /// For the handler method, use [`Record::query_one_handler()`].
    async fn query_one<I: IdParameter>(database: &Database, id_param: I) -> Option<Self> {
        Self::ReadRelation::query_one(database, id_param).await
    }

    /// Query (select) a single record from the database using an identifying key.
    ///
    /// If the record exists in the database, it is returned. Otherwise, [`None`] is returned.
    ///
    /// This is the Axum route handler version of this method. For the standard method, which can be
    /// called outside of an Axum context, see [`Record::query_one()`].
    async fn query_one_handler<I: IdParameter>(
        state: State<Arc<ServerState>>,
        id_param: Query<I>,
    ) -> Json<Option<Self>> {
        Self::ReadRelation::query_one_handler(state, id_param).await
    }

    #[allow(dead_code)]
    /// Query (select) all records for this relation from the database.
    ///
    /// This is the standard version of this method and should not be used as an Axum route handler.
    /// For the handler method, use [`Record::query_all_handler()`].
    async fn query_all(database: &Database) -> Self::ReadRelation {
        Self::ReadRelation::query_all(database).await
    }

    #[allow(dead_code)]
    /// Query (select) all records for this relation from the database.
    ///
    /// This is the Axum route handler version of this method. For the standard method, which can be
    /// called outside of an Axum context, see [`Record::query_all()`].
    async fn query_all_handler(state: State<Arc<ServerState>>) -> Json<Self::ReadRelation> {
        Self::ReadRelation::query_all_handler(state).await
    }
}
