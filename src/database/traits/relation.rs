use std::sync::Arc;

use axum::extract::{Json, Query, State};
use rand::{thread_rng, Rng};

use super::record::{Record, SingleInsert};
use crate::api::IdParameter;
use crate::database::{Database, SQL_PARAMETER_BIND_LIMIT};
use crate::ServerState;

/// A trait that allows table and view types to interoperate with and be queried from the database.
///
/// This does not implement any insertion or deletion methods because "relations" can be views,
/// which are read-only. For inserting items to tables, see the [`SingleInsert`] and [`BulkInsert`]
/// traits. For deleting items from tables, see the [`Table`] trait.
///
/// This trait does not do a lot on its own but it, along with [`Record`], provides the
/// functionality which allows almost all of the other database traits to be auto-implemented or
/// conveniently derived.
pub trait Relation: Sized {
    /// The record type which this relation contains a collection of.
    ///
    /// This type and the [`Record::Relation`] type are directly interreferential to allow
    /// "upcasting" and "downcasting," mostly for auto-implementations in other traits.
    type Record: Record<Relation = Self>;

    /// The name of the schema in which this relation exists in the database.
    ///
    /// This defaults to "main" but can be changed in case a relation lives in a different schema.
    /// The main alternate schema which would be used here is "persistent" for items which are not
    /// deleted each time the application is run. This will be unnecessary once Fixwise is no longer
    /// in early development/testing.
    const SCHEMA_NAME: &str = "main";
    /// The name of the relation in the database.
    ///
    /// It is recommended that all [`Relation`] types should have an identical name to the one they
    /// have in the database (with different case conventions, of course), but this is not assumed
    /// in order to be slightly less restrictive.
    const RELATION_NAME: &str;
    /// The primary column of this relation in the database.
    ///
    /// This is used directly in the SQL for querying the relation, so it should be in the format
    /// expected by SQL. For most relations, this will be a standalone column name, but for junction
    /// tables, it will be multiple column names written as a parenthesized, comma-separated list,
    /// such as `"(column_a, column_b, column_c)"`.
    const PRIMARY_KEY: &str;

    /// Create the relation from a collection of records.
    // TODO: Take `Into<Vec<Self::Record>>` here
    fn with_records(records: Vec<Self::Record>) -> Self;
    /// Convert the relation into a collection of records.
    fn take_records(self) -> Vec<Self::Record>;
    /// Borrow the relation's records.
    fn records(&self) -> &[Self::Record];

    /// Query (select) a single record from the database using an identifying key.
    ///
    /// If the record exists in the database, it is returned. Otherwise, [`None`] is returned.
    ///
    /// This is the standard version of this method and should not be used as an Axum route handler.
    /// For the handler method, use [`Relation::query_one_handler()`].
    async fn query_one<I: IdParameter>(database: &Database, id: I) -> Option<Self::Record> {
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
        State(state): State<Arc<ServerState>>,
        Query(id_param): Query<I>,
    ) -> Json<Option<Self::Record>> {
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
    async fn query_all_handler(State(state): State<Arc<ServerState>>) -> Json<Self> {
        Json(Self::query_all(&state.database).await)
    }

    /// Pick a random record from the relation.
    ///
    /// This is used mostly for randomly generating foreign keys, but can be used elsewhere if
    /// needed.
    fn pick_random(&self) -> Self::Record {
        let records = self.records();
        records[thread_rng().gen_range(0..records.len())].clone()
    }
}

/// A trait that allows table types to be deleted from the database.
///
/// For now, this trait only implements deletion methods, but it may be expanded in the future.
/// However, it is used as a trait bound for other traits which implement database write methods.
///
/// For querying items from tables, see the [`Relation`] trait. For inserting items to tables, see
/// the [`SingleInsert`] and [`BulkInsert`] traits.
pub trait Table: Relation {
    /// Delete a single record from the database using an identifying key.
    ///
    /// If the record is successfully deleted from the database, this method returns `true`. If an
    /// error occurs, such as if the record does not exist in the database, `false` is returned.
    ///
    /// This is the standard version of this method and should not be used as an Axum route handler.
    /// For the handler method, use [`Table::delete_one_handler()`].
    // TODO: Return a more useful value for error handling
    async fn delete_one<I: IdParameter>(database: &Database, id: I) -> bool {
        sqlx::query(&format!(
            "DELETE FROM {}.{} WHERE {} = $1",
            Self::SCHEMA_NAME,
            Self::RELATION_NAME,
            Self::PRIMARY_KEY,
        ))
        .bind(id.id() as i32)
        .execute(&database.connection)
        .await
        .is_ok()
    }

    /// Delete a single record from the database using an identifying key.
    ///
    /// If the record is successfully deleted from the database, this method returns `true`. If an
    /// error occurs, such as if the record does not exist in the database, `false` is returned.
    ///
    /// This is the Axum route handler version of this method. For the standard method, which can be
    /// called outside of an Axum context, see [`Table::delete_one()`].
    async fn delete_one_handler<I: IdParameter>(
        State(state): State<Arc<ServerState>>,
        Query(id_param): Query<I>,
    ) -> Json<bool> {
        Json(Self::delete_one(&state.database, id_param).await)
    }

    /// Delete all records for this relation from the database.
    ///
    /// If the records are successfully deleted from the database, this method returns `true`. If an
    /// error occurs, `false` is returned.
    ///
    /// This is the standard version of this method and should not be used as an Axum route handler.
    /// For the handler method, use [`Table::delete_all_handler()`].
    async fn delete_all(database: &Database) -> bool {
        sqlx::query(&format!(
            "DELETE FROM {}.{}",
            Self::SCHEMA_NAME,
            Self::RELATION_NAME,
        ))
        .execute(&database.connection)
        .await
        .is_ok()
    }

    /// Delete all records for this relation from the database.
    ///
    /// If the records are successfully deleted from the database, this method returns `true`. If an
    /// error occurs, `false` is returned.
    ///
    /// This is the Axum route handler version of this method. For the standard method, which can be
    /// called outside of an Axum context, see [`Table::delete_all()`].
    async fn delete_all_handler(State(state): State<Arc<ServerState>>) -> bool {
        Self::delete_all(&state.database).await
    }
}

/// A trait that allows an entire table of records to be inserted to the database in large batches.
///
/// Bulk-inserting items removes the need for establishing a network connection to the database
/// repeatedly. In initial testing, this proved to be about 20x more efficient than single insertion
/// when working with large tables. Of course, this is mostly used with synthetic data for testing
/// purposes, as it is relatively rare for a significant number of records to be inserted at once
/// during normal operation.
///
/// For single-insertion of records, see the related [`SingleInsert`] trait.
pub trait BulkInsert: Table<Record: SingleInsert> {
    /// The amount of records that can be inserted per batch/chunk.
    ///
    /// The batch limit is determined by the number of columns in a table. This is because a single
    /// SQL statement only supports up to [`u16::MAX`] parameter bindings, and each column takes up
    /// one parameter. Effectively, this means that tables with more columns are split into more
    /// batches, making bulk insertion take longer.
    const CHUNK_SIZE: usize = SQL_PARAMETER_BIND_LIMIT / Self::Record::COLUMN_NAMES.len();

    /// Convert a table of records into a series of batches to be inserted to the database.
    ///
    /// This method should only be used within auto-implementations.
    fn into_chunks(self) -> impl Iterator<Item = Vec<Self::Record>> {
        let mut iter = self.take_records().into_iter();
        // TODO: Annotate this code or something, I have very little idea what it does
        // * This was done because `itertools::IntoChunks` was causing issues with the axum handlers
        std::iter::from_fn(move || Some(iter.by_ref().take(Self::CHUNK_SIZE).collect()))
            .take_while(|v: &Vec<_>| !v.is_empty())
    }

    /// Insert the entire table into the database in a series of batches (or "chunks").
    ///
    /// This can insert tables of arbitrary size, but each batch is limited in size by number of
    /// parameters (table column count * record count).
    async fn insert_all(self, database: &Database) {
        for chunk in self.into_chunks() {
            let mut query_builder = Self::Record::get_query_builder();
            query_builder.push_values(chunk, Self::Record::push_column_bindings);
            database.execute_query_builder(query_builder).await;
        }
    }
}
