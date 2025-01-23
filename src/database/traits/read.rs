use std::sync::Arc;

use axum::extract::{Json, Query, State};

use super::shared::{Record, Relation};
#[allow(unused_imports)]
use super::write::{WriteRecord, WriteRelation};
use crate::api::IdParameter;
use crate::database::Database;
use crate::ServerState;

/// A trait that enables readable tables and views to have their records queried from the database.
///
/// This trait and [`WriteRelation`] are separated because because "relations" can be views, which
/// are read-only. Writable table types should also implement [`WriteRelation`].
///
/// This trait gets most of the information it needs to function from the upstream [`Relation`]
/// trait.
///
/// Also see [`ReadRecord`].
pub trait ReadRelation: Relation {
    /// The record type which this relation contains a collection of.
    ///
    /// This type and the [`ReadRecord::ReadRelation`] type are directly interreferential to allow
    /// convenient "upcasting" and "downcasting" so the relation and record types can be used
    /// interchangeably.
    ///
    /// This type is declared separately from [`Relation::Record`] because of cyclic dependency
    /// issues, but the type it refers to must be the same.
    type ReadRecord: ReadRecord<ReadRelation = Self>;

    /// Query (select) a single record from the database using an identifying key.
    ///
    /// If the record exists in the database, it is returned. Otherwise, [`None`] is returned.
    ///
    /// This is the standard version of this method and should not be used as an Axum route handler.
    /// For the handler method, use [`ReadRelation::query_one_handler()`].
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
    /// called outside of an Axum context, see [`ReadRelation::query_one()`].
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
    /// For the handler method, use [`ReadRelation::query_all_handler()`].
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
    /// called outside of an Axum context, see [`ReadRelation::query_all()`].
    async fn query_all_handler(state: State<Arc<ServerState>>) -> Json<Self> {
        Json(Self::query_all(&state.database).await)
    }
}

/// A trait that enables readable tables and views to have their records queried from the database.
///
/// This trait and [`WriteRecord`] are separated because because "relations" can be views, which
/// are read-only. Writable table record types should also implement [`WriteRecord`].
///
/// This trait gets most of the information it needs to function from the upstream [`Relation`]
/// trait, using the associated type [`Record::Relation`].
///
/// Also see [`ReadRelation`].
pub trait ReadRecord: Record {
    /// The relation type which contains a collection of this record type.
    ///
    /// This type and the [`ReadRelation::ReadRecord`] type are directly interreferential to allow
    /// convenient "upcasting" and "downcasting" so the relation and record types can be used
    /// interchangeably.
    ///
    /// This type is declared separately from [`Record::Relation`] because of cyclic dependency
    /// issues, but the type it refers to must be the same.
    type ReadRelation: ReadRelation<ReadRecord = Self>;

    #[allow(dead_code)]
    /// Query (select) a single record from the database using an identifying key.
    ///
    /// If the record exists in the database, it is returned. Otherwise, [`None`] is returned.
    ///
    /// This is the standard version of this method and should not be used as an Axum route handler.
    /// For the handler method, use [`ReadRecord::query_one_handler()`].
    async fn query_one<I: IdParameter>(database: &Database, id_param: I) -> Option<Self> {
        Self::ReadRelation::query_one(database, id_param).await
    }

    /// Query (select) a single record from the database using an identifying key.
    ///
    /// If the record exists in the database, it is returned. Otherwise, [`None`] is returned.
    ///
    /// This is the Axum route handler version of this method. For the standard method, which can be
    /// called outside of an Axum context, see [`ReadRecord::query_one()`].
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
    /// For the handler method, use [`ReadRecord::query_all_handler()`].
    async fn query_all(database: &Database) -> Self::ReadRelation {
        Self::ReadRelation::query_all(database).await
    }

    #[allow(dead_code)]
    /// Query (select) all records for this relation from the database.
    ///
    /// This is the Axum route handler version of this method. For the standard method, which can be
    /// called outside of an Axum context, see [`ReadRecord::query_all()`].
    async fn query_all_handler(state: State<Arc<ServerState>>) -> Json<Self::ReadRelation> {
        Self::ReadRelation::query_all_handler(state).await
    }
}
