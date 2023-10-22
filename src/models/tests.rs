use std::collections::HashSet;

use super::common::{
    Device, DeviceClassification, DeviceClassificationUniqueID, DeviceID, DeviceManufacturer,
    DeviceManufacturerUniqueID, InventoryExtensionUniqueID, UniqueID,
};

impl DeviceManufacturer {
    /// Creates a basic device manufacturer for testing purposes.
    /// Can be modified to test different scenarios.
    pub fn test(num: u32, extension_id: &InventoryExtensionUniqueID) -> Self {
        Self {
            id: DeviceManufacturerUniqueID::new(format!("test_{num}")),
            common_name: format!("Test Device Manufacturer {num}"),
            extensions: HashSet::from([extension_id.clone()]),
        }
    }
}

impl DeviceClassification {
    /// Creates a basic device classification for testing purposes.
    /// Can be modified to test different scenarios.
    pub fn test(num: u32, extension_id: &InventoryExtensionUniqueID) -> Self {
        Self {
            id: DeviceClassificationUniqueID::new(format!("test_{num}")),
            common_name: format!("Test Device Classification {num}"),
            extensions: HashSet::from([extension_id.clone()]),
        }
    }
}

impl Device {
    /// Creates a basic device for testing purposes.
    /// Can be modified to test different scenarios.
    pub fn test(
        num: u32,
        extension_id: &InventoryExtensionUniqueID,
        manufacturer_id: &DeviceManufacturerUniqueID,
        classification_id: &DeviceClassificationUniqueID,
    ) -> Self {
        Self {
            id: DeviceID::new(
                extension_id.unnamespaced(),
                manufacturer_id.unnamespaced(),
                classification_id.unnamespaced(),
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
