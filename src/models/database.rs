use serde::{Deserialize, Serialize};
use surrealdb::sql::Thing;

/// The metadata of an extension which can be added to the database.
#[derive(Debug, Serialize)]
pub struct InventoryExtensionMetadataPushRecord<'a> {
    pub id: Thing,
    pub common_name: &'a str,
    pub version: String,
}

/// The metadata of an extension as read from the database.
#[derive(Debug, Deserialize)]
pub struct InventoryExtensionMetadataPullRecord {
    pub id: Thing,
    pub common_name: String,
    pub version: String,
}

/// A device manufacturer which can be added to the database.
#[derive(Debug, Serialize)]
pub struct ManufacturerPushRecord<'a> {
    pub id: Thing,
    pub common_name: &'a str,
    pub extensions: Vec<Thing>,
}

/// A device manufacturer as read from the database.
#[derive(Debug, Deserialize)]
pub struct ManufacturerPullRecord {
    pub id: Thing,
    pub common_name: String,
    pub extensions: Vec<Thing>,
}

/// A classification of device which can be added to the database.
#[derive(Debug, Serialize)]
pub struct ClassificationPushRecord<'a> {
    pub id: Thing,
    pub common_name: &'a str,
    pub extensions: Vec<Thing>,
}

/// A classification of device as read from the database.
#[derive(Debug, Deserialize)]
pub struct ClassificationPullRecord {
    pub id: Thing,
    pub common_name: String,
    pub extensions: Vec<Thing>,
}

/// A device and all of its relevant metadata, which can be added to the database.
#[derive(Debug, Serialize)]
pub struct DevicePushRecord<'a> {
    pub id: Thing,
    pub common_name: &'a str,
    pub manufacturer: Thing,
    pub classification: Thing,
    pub extension: Thing,
    pub primary_model_identifiers: &'a [String],
    pub extended_model_identifiers: &'a [String],
}

/// A device and all of its relevant metadata, as read from the database.
#[derive(Debug, Deserialize)]
pub struct DevicePullRecord {
    pub id: Thing,
    pub common_name: String,
    pub manufacturer: Thing,
    pub classification: Thing,
    pub extension: Thing,
    pub primary_model_identifiers: Vec<String>,
    pub extended_model_identifiers: Vec<String>,
}

/// A record with the bare minimum amount of structure, used to provide generic functions
/// (particularly [`Surreal::create`](surrealdb::Surreal::create) with type parameters when the actual result is not important.
#[derive(Debug, Deserialize)]
pub struct GenericPullRecord {
    // * This has to be an attribute tag because `_id` does not map to `id`.
    #[allow(dead_code)]
    id: Thing,
}
