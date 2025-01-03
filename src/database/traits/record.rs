use std::sync::Arc;

use axum::extract::{Json, Query, State};
use sqlx::postgres::PgRow;
use sqlx::query_builder::{QueryBuilder, Separated};
use sqlx::Postgres;

#[allow(unused_imports)]
use super::relation::{BulkInsert, Relation, Table};
use crate::api::IdParameter;
use crate::database::Database;
use crate::ServerState;

/// A trait that allows table/view record types to interoperate with and be queried from the
/// database.
///
/// This does not implement any insertion methods because "relations" can be views, which are
/// read-only. For inserting items to tables, see the [`SingleInsert`] and [`BulkInsert`] traits.
///
/// This trait mostly exists for use with insertion traits, but also acts as a passthrough to allow
/// items to be queried using the record type instead of the relation type when convenient.
pub trait Record: for<'a> sqlx::FromRow<'a, PgRow> + Send + Unpin + Clone {
    /// The relation type which contains a collection of this record type.
    ///
    /// This type and the [`Relation::Record`] type are directly interreferential to allow
    /// "upcasting" and "downcasting," mostly for auto-implementations in other traits.
    type Relation: Relation<Record = Self>;

    #[allow(dead_code)]
    /// Query (select) a single record from the database using an identifying key.
    ///
    /// If the record exists in the database, it is returned. Otherwise, [`None`] is returned.
    ///
    /// This is the standard version of this method and should not be used as an Axum route handler.
    /// For the handler method, use [`Record::query_one_handler()`].
    async fn query_one<I: IdParameter>(database: &Database, id_param: I) -> Option<Self> {
        Self::Relation::query_one(database, id_param).await
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
        Self::Relation::query_one_handler(state, id_param).await
    }

    #[allow(dead_code)]
    /// Query (select) all records for this relation from the database.
    ///
    /// This is the standard version of this method and should not be used as an Axum route handler.
    /// For the handler method, use [`Record::query_all_handler()`].
    async fn query_all(database: &Database) -> Self::Relation {
        Self::Relation::query_all(database).await
    }

    #[allow(dead_code)]
    /// Query (select) all records for this relation from the database.
    ///
    /// This is the Axum route handler version of this method. For the standard method, which can be
    /// called outside of an Axum context, see [`Record::query_all()`].
    async fn query_all_handler(state: State<Arc<ServerState>>) -> Json<Self::Relation> {
        Self::Relation::query_all_handler(state).await
    }
}

pub trait TableRecord: Record<Relation: Table> {
    #[allow(dead_code)]
    /// Delete a single record from the database using an identifying key.
    ///
    /// If the record is successfully deleted from the database, this method returns `true`. If an
    /// error occurs, such as if the record does not exist in the database, `false` is returned.
    ///
    /// This is the standard version of this method and should not be used as an Axum route handler.
    /// For the handler method, use [`TableRecord::delete_one_handler()`].
    async fn delete_one<I: IdParameter>(database: &Database, id: I) -> bool {
        Self::Relation::delete_one(database, id).await
    }

    #[allow(dead_code)]
    /// Delete a single record from the database using an identifying key.
    ///
    /// If the record is successfully deleted from the database, this method returns `true`. If an
    /// error occurs, such as if the record does not exist in the database, `false` is returned.
    ///
    /// This is the Axum route handler version of this method. For the standard method, which can be
    /// called outside of an Axum context, see [`TableRecord::delete_one()`].
    async fn delete_one_handler<I: IdParameter>(
        state: State<Arc<ServerState>>,
        id_param: Query<I>,
    ) -> Json<bool> {
        Self::Relation::delete_one_handler(state, id_param).await
    }

    #[allow(dead_code)]
    /// Delete all records for this relation from the database.
    ///
    /// If the records are successfully deleted from the database, this method returns `true`. If an
    /// error occurs, `false` is returned.
    ///
    /// This is the standard version of this method and should not be used as an Axum route handler.
    /// For the handler method, use [`TableRecord::delete_all_handler()`].
    async fn delete_all(database: &Database) -> bool {
        Self::Relation::delete_all(database).await
    }

    #[allow(dead_code)]
    /// Delete all records for this relation from the database.
    ///
    /// If the records are successfully deleted from the database, this method returns `true`. If an
    /// error occurs, `false` is returned.
    ///
    /// This is the Axum route handler version of this method. For the standard method, which can be
    /// called outside of an Axum context, see [`TableRecord::delete_all()`].
    async fn delete_all_handler(state: State<Arc<ServerState>>) -> bool {
        Self::Relation::delete_all_handler(state).await
    }
}

/// A trait that allows a single record to be inserted to the database.
///
/// Though it would be possible to make this trait generic over [`Record`], it is only meant to be
/// implemented on [`TableRecord`] types, as items cannot be inserted into a database view.
///
/// For bulk-insertion of records, see the related [`BulkInsert`] trait.
pub trait SingleInsert: TableRecord {
    /// The names of all columns in the database table.
    ///
    /// This was going to be a member of [`Table`] but was placed here because it is needed for
    /// [`SingleInsert::get_query_builder`] to generate the SQL for inserting records to the
    /// database, as well as determining the [`BulkInsert::CHUNK_SIZE`].
    const COLUMN_NAMES: &[&str];

    /// Get the [`QueryBuilder`] necessary to insert one or more records of data into the database.
    ///
    /// This is used by both [`SingleInsert`] and [`BulkInsert`] and is meant mostly for
    /// auto-implementations.
    fn get_query_builder<'a>() -> QueryBuilder<'a, Postgres> {
        QueryBuilder::new(&format!(
            "INSERT INTO {}.{} ({}) ",
            Self::Relation::SCHEMA_NAME,
            Self::Relation::RELATION_NAME,
            Self::COLUMN_NAMES.join(", ")
        ))
    }

    /// Push the record's data into the [`QueryBuilder`] so it can be built and executed against the
    /// database.
    ///
    /// This method is used as a function parameter for [`QueryBuilder::push_values`] and should
    /// only be used within auto-implementations.
    fn push_column_bindings(builder: Separated<Postgres, &str>, record: Self);

    /// Insert the record into the database.
    ///
    /// This should not be used repeatedly for a collection of records. Inserting multiple records
    /// can be done much more efficiently using [`BulkInsert::insert_all`], which should be
    /// implemented for any database table type.
    async fn insert(self, database: &Database) {
        let mut query_builder = Self::get_query_builder();
        query_builder.push_values(std::iter::once(self), Self::push_column_bindings);
        database.execute_query_builder(query_builder).await;
    }
}
