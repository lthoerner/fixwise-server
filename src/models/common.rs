use std::collections::HashSet;

use semver::Version;

use crate::database::{
    CLASSIFICATION_TABLE_NAME, DEVICE_TABLE_NAME, EXTENSION_TABLE_NAME, MANUFACTURER_TABLE_NAME,
};

/// An explicitly-namespaced extension ID.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct InventoryExtensionID {
    pub non_namespaced_id: String,
}

/// An explicitly-namespaced manufacturer ID.
#[derive(Debug, Clone, PartialEq)]
pub struct ManufacturerID {
    pub non_namespaced_id: String,
}

/// An explicitly-namespaced classification ID.
#[derive(Debug, Clone, PartialEq)]
pub struct ClassificationID {
    pub non_namespaced_id: String,
}

/// An explicitly-namespaced device ID, in the format of
/// `<extension>/<manufacturer>/<classification>/<device>`.
/// This allows for devices which have different extensions, manufacturers, or classifications to
/// share the same name, and for duplicates to be easily identified.
/// The extension and classification IDs are not namespaced to their respective tables in this form.
#[derive(Debug, Clone, PartialEq)]
pub struct DeviceID {
    pub extension_id: InventoryExtensionID,
    pub manufacturer_id: ManufacturerID,
    pub classification_id: ClassificationID,
    pub non_namespaced_id: String,
}

/// The metadata of an extension.
/// This does not include the extension contents, such as devices or manufacturers.
/// It is used to identify existing extensions to the `ExtensionManager` to prevent conflicts.
#[derive(Debug, PartialEq)]
pub struct InventoryExtensionInfo {
    pub id: InventoryExtensionID,
    pub common_name: String,
    pub version: Version,
}

/// A device manufacturer.
#[derive(Debug, Clone, PartialEq)]
pub struct Manufacturer {
    pub id: ManufacturerID,
    pub common_name: String,
    pub extensions: HashSet<InventoryExtensionID>,
}

/// A classification of device, such as a phone, tablet, or gaming console.
#[derive(Debug, Clone, PartialEq)]
pub struct Classification {
    pub id: ClassificationID,
    pub common_name: String,
    pub extensions: HashSet<InventoryExtensionID>,
}

/// A device and all of its relevant metadata, such as its make and model.
#[derive(Debug, Clone, PartialEq)]
pub struct Device {
    pub id: DeviceID,
    pub common_name: String,
    pub manufacturer: ManufacturerID,
    pub classification: ClassificationID,
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

impl ManufacturerID {
    pub fn new(id: &str) -> Self {
        Self {
            non_namespaced_id: id.to_owned(),
        }
    }

    pub fn to_non_namespaced_string(&self) -> String {
        self.non_namespaced_id.clone()
    }

    pub fn to_namespaced_string(&self) -> String {
        [MANUFACTURER_TABLE_NAME, &self.non_namespaced_id].join(":")
    }
}

impl ClassificationID {
    pub fn new(id: &str) -> Self {
        Self {
            non_namespaced_id: id.to_owned(),
        }
    }

    pub fn to_non_namespaced_string(&self) -> String {
        self.non_namespaced_id.clone()
    }

    pub fn to_namespaced_string(&self) -> String {
        [CLASSIFICATION_TABLE_NAME, &self.non_namespaced_id].join(":")
    }
}

impl DeviceID {
    /// Creates a `DeviceID`.
    /// None of the parameters to this function should be namespaced.
    pub fn new(
        extension_id: &str,
        manufacturer_id: &str,
        classification_id: &str,
        id: &str,
    ) -> Self {
        Self {
            extension_id: InventoryExtensionID::new(extension_id),
            manufacturer_id: ManufacturerID::new(manufacturer_id),
            classification_id: ClassificationID::new(classification_id),
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

impl Manufacturer {
    /// Creates a basic manufacturer for testing purposes.
    /// Can be modified to test different scenarios.
    #[cfg(test)]
    #[allow(dead_code)]
    pub fn test(num: u32, extension_id: &InventoryExtensionID) -> Self {
        Self {
            id: ManufacturerID::new(&format!("test_{num}")),
            common_name: format!("Test Manufacturer {num}"),
            extensions: HashSet::from([extension_id.clone()]),
        }
    }

    /// Merges the extensions field of another manufacturer into this one.
    /// Does not check whether the two manufacturers share the same ID and other metadata.
    pub fn merge(&mut self, other: Manufacturer) {
        self.extensions.extend(other.extensions);
    }
}

impl Classification {
    /// Creates a basic classification for testing purposes.
    /// Can be modified to test different scenarios.
    #[cfg(test)]
    #[allow(dead_code)]
    pub fn test(num: u32, extension_id: &InventoryExtensionID) -> Self {
        Self {
            id: ClassificationID::new(&format!("test_{num}")),
            common_name: format!("Test Classification {num}"),
            extensions: HashSet::from([extension_id.clone()]),
        }
    }

    /// Merges the extensions field of another classification into this one.
    /// Does not check whether the two classifications share the same ID and other metadata.
    pub fn merge(&mut self, other: Classification) {
        self.extensions.extend(other.extensions);
    }
}

impl Device {
    /// Creates a basic device for testing purposes.
    /// Can be modified to test different scenarios.
    #[cfg(test)]
    #[allow(dead_code)]
    pub fn test(
        num: u32,
        extension_id: &InventoryExtensionID,
        manufacturer_id: &ManufacturerID,
        classification_id: &ClassificationID,
    ) -> Self {
        Self {
            id: DeviceID::new(
                &extension_id.to_non_namespaced_string(),
                &manufacturer_id.to_non_namespaced_string(),
                &classification_id.to_non_namespaced_string(),
                &format!("test_{num}"),
            ),
            common_name: format!("Test Device {num}"),
            manufacturer: manufacturer_id.clone(),
            classification: classification_id.clone(),
            extension: extension_id.clone(),
            primary_model_identifiers: vec![format!("test_{num}_primary")],
            extended_model_identifiers: vec![format!("test_{num}_extended")],
        }
    }
}
