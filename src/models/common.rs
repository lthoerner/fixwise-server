use std::collections::HashSet;

use semver::Version;

use crate::database::{
    DEVICE_CLASSIFICATION_TABLE_NAME, DEVICE_MANUFACTURER_TABLE_NAME, DEVICE_TABLE_NAME,
    EXTENSION_TABLE_NAME,
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

/// An explicitly-namespaced device ID, in the format of
/// `<extension>/<manufacturer>/<classification>/<device>`.
/// This allows for devices which have different extensions, manufacturers, or classifications to
/// share the same name, and for duplicates to be easily identified.
/// The extension, device manufacturer, and device classification IDs are not namespaced to their
/// respective tables in this form.
#[derive(Debug, Clone, PartialEq)]
pub struct DeviceID {
    pub extension_id: InventoryExtensionUniqueID,
    pub manufacturer_id: DeviceManufacturerUniqueID,
    pub classification_id: DeviceClassificationUniqueID,
    non_namespaced_id: String,
}

/// The metadata of an inventory extension.
/// This does not include the extension contents, such as devices or manufacturers.
/// Used to identify existing extensions to the
/// [`ExtensionManager`](crate::extensions::ExtensionManager) to prevent conflicts.
#[derive(Debug, Clone, PartialEq)]
pub struct InventoryExtensionMetadata {
    pub id: InventoryExtensionUniqueID,
    pub common_name: String,
    pub version: Version,
}

/// A device manufacturer.
#[derive(Debug, Clone, PartialEq)]
pub struct DeviceManufacturer {
    pub id: DeviceManufacturerUniqueID,
    pub common_name: String,
    pub extensions: HashSet<InventoryExtensionUniqueID>,
}

/// A classification of device, such as a phone, tablet, or gaming console.
#[derive(Debug, Clone, PartialEq)]
pub struct DeviceClassification {
    pub id: DeviceClassificationUniqueID,
    pub common_name: String,
    pub extensions: HashSet<InventoryExtensionUniqueID>,
}

/// A device and all of its relevant metadata, such as its make and model.
#[derive(Debug, Clone, PartialEq)]
pub struct Device {
    pub id: DeviceID,
    pub common_name: String,
    pub manufacturer: DeviceManufacturerUniqueID,
    pub classification: DeviceClassificationUniqueID,
    pub extension: InventoryExtensionUniqueID,
    pub primary_model_identifiers: Vec<String>,
    pub extended_model_identifiers: Vec<String>,
}

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

impl DeviceID {
    /// Creates a [`DeviceID`].
    /// None of the parameters to this function should be namespaced.
    pub fn new(
        extension_id: &str,
        manufacturer_id: &str,
        classification_id: &str,
        id: &str,
    ) -> Self {
        Self {
            extension_id: InventoryExtensionUniqueID::new(extension_id),
            manufacturer_id: DeviceManufacturerUniqueID::new(manufacturer_id),
            classification_id: DeviceClassificationUniqueID::new(classification_id),
            non_namespaced_id: id.to_owned(),
        }
    }

    pub fn to_non_namespaced_string(&self) -> String {
        [
            self.extension_id.unnamespaced(),
            self.manufacturer_id.unnamespaced(),
            self.classification_id.unnamespaced(),
            &self.non_namespaced_id,
        ]
        .join("/")
    }

    pub fn to_namespaced_string(&self) -> String {
        [DEVICE_TABLE_NAME, &self.to_non_namespaced_string()].join(":")
    }
}

impl DeviceManufacturer {
    /// Merges the extensions field of another device manufacturer into this one.
    /// Does not check whether the two device manufacturers share the same ID and other metadata.
    pub fn merge(&mut self, other: DeviceManufacturer) {
        self.extensions.extend(other.extensions);
    }
}

impl DeviceClassification {
    /// Merges the extensions field of another device classification into this one.
    /// Does not check whether the two device classifications share the same ID and other metadata.
    pub fn merge(&mut self, other: DeviceClassification) {
        self.extensions.extend(other.extensions);
    }
}
