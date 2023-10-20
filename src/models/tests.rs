use std::collections::HashSet;

use super::common::{
    Classification, ClassificationID, Device, DeviceID, InventoryExtensionID, Manufacturer,
    ManufacturerID,
};

impl Manufacturer {
    /// Creates a basic manufacturer for testing purposes.
    /// Can be modified to test different scenarios.
    pub fn test(num: u32, extension_id: &InventoryExtensionID) -> Self {
        Self {
            id: ManufacturerID::new(&format!("test_{num}")),
            common_name: format!("Test Manufacturer {num}"),
            extensions: HashSet::from([extension_id.clone()]),
        }
    }
}

impl Classification {
    /// Creates a basic classification for testing purposes.
    /// Can be modified to test different scenarios.
    pub fn test(num: u32, extension_id: &InventoryExtensionID) -> Self {
        Self {
            id: ClassificationID::new(&format!("test_{num}")),
            common_name: format!("Test Classification {num}"),
            extensions: HashSet::from([extension_id.clone()]),
        }
    }
}

impl Device {
    /// Creates a basic device for testing purposes.
    /// Can be modified to test different scenarios.
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
