use crate::database::{
    DEVICE_CLASSIFICATION_TABLE_NAME, DEVICE_MANUFACTURER_TABLE_NAME, EXTENSION_TABLE_NAME,
};

/// A trait for ID types which are used as "primary keys" (unique string identifiers) in the
/// database, as opposed to Surreal's auto-generated UUIDs (used for non-unique items).
pub trait UniqueID {
    const TABLE_NAME: &'static str;
    fn new(id: impl Into<String>) -> Self;
    fn namespaced(&self) -> String;
    fn unnamespaced(&self) -> &str;
}

/// An unnamespaced unique extension ID.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct InventoryExtensionUniqueID(String);

/// An unnamespaced unique device manufacturer ID.
#[derive(Debug, Clone, PartialEq)]
pub struct DeviceManufacturerUniqueID(String);

/// An unnamespaced unique device classification ID.
#[derive(Debug, Clone, PartialEq)]
pub struct DeviceClassificationUniqueID(String);

impl UniqueID for InventoryExtensionUniqueID {
    const TABLE_NAME: &'static str = EXTENSION_TABLE_NAME;
    fn new(id: impl Into<String>) -> Self {
        Self(id.into())
    }

    fn namespaced(&self) -> String {
        [Self::TABLE_NAME, &self.0].join(":")
    }

    fn unnamespaced(&self) -> &str {
        &self.0
    }
}

impl UniqueID for DeviceManufacturerUniqueID {
    const TABLE_NAME: &'static str = DEVICE_MANUFACTURER_TABLE_NAME;
    fn new(id: impl Into<String>) -> Self {
        Self(id.into())
    }

    fn namespaced(&self) -> String {
        [Self::TABLE_NAME, &self.0].join(":")
    }

    fn unnamespaced(&self) -> &str {
        &self.0
    }
}

impl UniqueID for DeviceClassificationUniqueID {
    const TABLE_NAME: &'static str = DEVICE_CLASSIFICATION_TABLE_NAME;
    fn new(id: impl Into<String>) -> Self {
        Self(id.into())
    }

    fn namespaced(&self) -> String {
        [Self::TABLE_NAME, &self.0].join(":")
    }

    fn unnamespaced(&self) -> &str {
        &self.0
    }
}
