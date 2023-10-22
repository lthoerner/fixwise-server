pub mod ids;

use std::collections::HashSet;

use semver::Version;

use self::ids::{
    DeviceClassificationUniqueID, DeviceID, DeviceManufacturerUniqueID, InventoryExtensionUniqueID,
};

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
