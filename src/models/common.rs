use std::collections::HashSet;

use semver::Version;

use crate::database::{
    DEVICE_CLASSIFICATION_TABLE_NAME, DEVICE_MANUFACTURER_TABLE_NAME, DEVICE_TABLE_NAME,
    EXTENSION_TABLE_NAME,
};

/// An explicitly-namespaced extension ID.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct InventoryExtensionID {
    non_namespaced_id: String,
}

/// An explicitly-namespaced device manufacturer ID.
#[derive(Debug, Clone, PartialEq)]
pub struct DeviceManufacturerID {
    non_namespaced_id: String,
}

/// An explicitly-namespaced device classification ID.
#[derive(Debug, Clone, PartialEq)]
pub struct DeviceClassificationID {
    non_namespaced_id: String,
}

/// An explicitly-namespaced device ID, in the format of
/// `<extension>/<manufacturer>/<classification>/<device>`.
/// This allows for devices which have different extensions, manufacturers, or classifications to
/// share the same name, and for duplicates to be easily identified.
/// The extension, device manufacturer, and device classification IDs are not namespaced to their
/// respective tables in this form.
#[derive(Debug, Clone, PartialEq)]
pub struct DeviceID {
    pub extension_id: InventoryExtensionID,
    pub manufacturer_id: DeviceManufacturerID,
    pub classification_id: DeviceClassificationID,
    non_namespaced_id: String,
}

/// The metadata of an inventory extension.
/// This does not include the extension contents, such as devices or manufacturers.
/// Used to identify existing extensions to the
/// [`ExtensionManager`](crate::extensions::ExtensionManager) to prevent conflicts.
#[derive(Debug, Clone, PartialEq)]
pub struct InventoryExtensionMetadata {
    pub id: InventoryExtensionID,
    pub common_name: String,
    pub version: Version,
}

/// A device manufacturer.
#[derive(Debug, Clone, PartialEq)]
pub struct DeviceManufacturer {
    pub id: DeviceManufacturerID,
    pub common_name: String,
    pub extensions: HashSet<InventoryExtensionID>,
}

/// A classification of device, such as a phone, tablet, or gaming console.
#[derive(Debug, Clone, PartialEq)]
pub struct DeviceClassification {
    pub id: DeviceClassificationID,
    pub common_name: String,
    pub extensions: HashSet<InventoryExtensionID>,
}

/// A device and all of its relevant metadata, such as its make and model.
#[derive(Debug, Clone, PartialEq)]
pub struct Device {
    pub id: DeviceID,
    pub common_name: String,
    pub manufacturer: DeviceManufacturerID,
    pub classification: DeviceClassificationID,
    pub extension: InventoryExtensionID,
    pub primary_model_identifiers: Vec<String>,
    pub extended_model_identifiers: Vec<String>,
}

impl InventoryExtensionID {
    pub fn new(id: &str) -> Self {
        Self {
            non_namespaced_id: id.to_owned(),
        }
    }

    pub fn to_non_namespaced_string(&self) -> String {
        self.non_namespaced_id.clone()
    }

    pub fn to_namespaced_string(&self) -> String {
        [EXTENSION_TABLE_NAME, &self.non_namespaced_id].join(":")
    }
}

impl DeviceManufacturerID {
    pub fn new(id: &str) -> Self {
        Self {
            non_namespaced_id: id.to_owned(),
        }
    }

    pub fn to_non_namespaced_string(&self) -> String {
        self.non_namespaced_id.clone()
    }

    pub fn to_namespaced_string(&self) -> String {
        [DEVICE_MANUFACTURER_TABLE_NAME, &self.non_namespaced_id].join(":")
    }
}

impl DeviceClassificationID {
    pub fn new(id: &str) -> Self {
        Self {
            non_namespaced_id: id.to_owned(),
        }
    }

    pub fn to_non_namespaced_string(&self) -> String {
        self.non_namespaced_id.clone()
    }

    pub fn to_namespaced_string(&self) -> String {
        [DEVICE_CLASSIFICATION_TABLE_NAME, &self.non_namespaced_id].join(":")
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
            extension_id: InventoryExtensionID::new(extension_id),
            manufacturer_id: DeviceManufacturerID::new(manufacturer_id),
            classification_id: DeviceClassificationID::new(classification_id),
            non_namespaced_id: id.to_owned(),
        }
    }

    pub fn to_non_namespaced_string(&self) -> String {
        [
            self.extension_id.to_non_namespaced_string().as_str(),
            self.manufacturer_id.to_non_namespaced_string().as_str(),
            self.classification_id.to_non_namespaced_string().as_str(),
            self.non_namespaced_id.as_str(),
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
