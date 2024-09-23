pub mod endpoints;

use std::sync::Arc;

use axum::extract::{Query, State};
use axum::Json;
use serde::{Deserialize, Serialize};

use proc_macros::IdParameter;

use crate::database::{DatabaseEntity, DatabaseRow};
use crate::ServerState;

/// A trait that allows a JSON collection endpoint to be served to the API.
///
/// How this endpoint behaves is entirely dependent on the implementation of [`FromDatabaseEntity`],
/// which converts the data from the database into the format used by the API. See its documentation
/// for more details.
pub trait ServeEntityJson: FromDatabaseEntity + Serialize + Sized {
    /// Serve a JSON collection endpoint.
    ///
    /// This function is used as an axum handler via [`axum::routing::method_routing::get`].
    async fn serve_all(state: State<Arc<ServerState>>) -> Json<Self> {
        Json(Self::from_database_entity(
            Self::Entity::query_all_handler(state).await,
        ))
    }
}

/// A trait that allows a JSON record endpoint to be served to the API.
///
/// How this endpoint behaves is entirely dependent on the implementation of [`FromDatabaseRow`],
/// which converts the data from the database into the format used by the API. This is implemented
/// independently from [`ServeEntityJson`] and implementors need not have a collection (entity) type
/// associated with them on the API side.
///
/// This may need to be changed eventually to use a different underlying trait than
/// [`FromDatabaseRow`], as some [`ServeRowJson::serve_one`] endpoints may need to send different
/// row data than their [`ServeEntityJson::serve_all`] counterparts. This can be accomplished by
/// writing separate row-level conversions into a [`FromDatabaseEntity`] implementation, but it
/// would be better for separation of concerns.
pub trait ServeRowJson<I: IdParameter>: FromDatabaseRow + Serialize + Sized {
    /// Serve a JSON record endpoint.
    ///
    /// This function is used as an axum handler via [`axum::routing::method_routing::get`].
    async fn serve_one(state: State<Arc<ServerState>>, id_param: Query<I>) -> Json<Option<Self>> {
        Json(Some(Self::from_database_row(
            Self::Row::query_one_handler(state, id_param).await.unwrap(),
        )))
    }
}

/// A trait that allows a database table or view to be converted into data that can be used by an
/// API endpoint.
///
/// This can act as a simple thin wrapper for the raw JSON, but it is often used to return data with
/// additional structure and formatting instructions for the frontend to work with. Oftentimes this
/// trait implementation will just wrap repeated calls to [`FromDatabaseRow::from_database_row`],
/// but in some cases it may be desirable for rows to be converted differently than their
/// corresponding table or view.
pub trait FromDatabaseEntity {
    /// The table or view type in the database to be converted by
    /// [`FromDatabaseEntity::from_database_entity`].
    type Entity: DatabaseEntity;

    /// Convert the database entity into the data required for the endpoint.
    fn from_database_entity(entity: Self::Entity) -> Self;
}

/// A trait that allows a database row to be converted into data that can be used by an API
/// endpoint.
///
/// This can act as a simple thin wrapper for the raw JSON, but it is often used to return data with
/// additional structure and formatting instructions for the frontend to work with.
pub trait FromDatabaseRow {
    /// The row type in the database to be converted by [`FromDatabaseRow::from_database_row`].
    type Row: DatabaseRow;

    /// Convert the database row into the data required for the endpoint.
    fn from_database_row(row: Self::Row) -> Self;
}

/// A trait that allows queries including an ID field to use unique nomenclature if desired.
///
/// The format for the URL will look like
/// `https://fixwise.io/some/record/endpoint?id_parameter_name=123456`. If the ID parameter is
/// just named `id`, simply use [`GenericIdParameter`].
pub trait IdParameter {
    /// Create the parameter with an inner [`usize`].
    fn new(value: usize) -> Self;
    /// Get the inner [`usize`] ID parameter.
    fn id(&self) -> usize;
}

/// A generic ID query parameter type.
///
/// Endpoints using this ID parameter will have URLs like
/// `https://fixwise.io/some/record/endpoint?id=123456`.
#[derive(Clone, Deserialize, IdParameter)]
pub struct GenericIdParameter {
    id: usize,
}
