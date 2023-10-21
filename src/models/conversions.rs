use std::collections::HashSet;

use anyhow::anyhow;
use semver::Version;
use surrealdb::sql::{Id, Thing};

use super::common::{
    Device, DeviceClassification, DeviceClassificationID, DeviceID, DeviceManufacturer,
    DeviceManufacturerID, InventoryExtensionID, InventoryExtensionMetadata,
};
use super::database::{
    DeviceClassificationPullRecord, DeviceClassificationPushRecord, DeviceManufacturerPullRecord,
    DeviceManufacturerPushRecord, DevicePullRecord, DevicePushRecord,
    InventoryExtensionMetadataPullRecord, InventoryExtensionMetadataPushRecord,
};
use crate::database::{
    DEVICE_CLASSIFICATION_TABLE_NAME, DEVICE_MANUFACTURER_TABLE_NAME, DEVICE_TABLE_NAME,
    EXTENSION_TABLE_NAME,
};

impl<'a> From<&'a InventoryExtensionMetadata> for InventoryExtensionMetadataPushRecord<'a> {
    fn from(extension: &'a InventoryExtensionMetadata) -> Self {
        InventoryExtensionMetadataPushRecord {
            id: Thing::from(&extension.id),
            common_name: &extension.common_name,
            version: extension.version.to_string(),
        }
    }
}

impl TryFrom<InventoryExtensionMetadataPullRecord> for InventoryExtensionMetadata {
    type Error = anyhow::Error;
    fn try_from(extension: InventoryExtensionMetadataPullRecord) -> Result<Self, anyhow::Error> {
        Ok(InventoryExtensionMetadata {
            id: InventoryExtensionID::try_from(extension.id)?,
            common_name: extension.common_name,
            version: Version::parse(&extension.version)?,
        })
    }
}

impl<'a> From<&'a DeviceManufacturer> for DeviceManufacturerPushRecord<'a> {
    fn from(manufacturer: &'a DeviceManufacturer) -> Self {
        DeviceManufacturerPushRecord {
            id: Thing::from(&manufacturer.id),
            common_name: &manufacturer.common_name,
            extensions: manufacturer.extensions.iter().map(Thing::from).collect(),
        }
    }
}

impl TryFrom<DeviceManufacturerPullRecord> for DeviceManufacturer {
    type Error = anyhow::Error;
    fn try_from(manufacturer: DeviceManufacturerPullRecord) -> Result<Self, anyhow::Error> {
        Ok(DeviceManufacturer {
            id: DeviceManufacturerID::try_from(manufacturer.id)?,
            common_name: manufacturer.common_name,
            extensions: manufacturer
                .extensions
                .into_iter()
                .map(InventoryExtensionID::try_from)
                .collect::<Result<HashSet<_>, _>>()?,
        })
    }
}

impl<'a> From<&'a DeviceClassification> for DeviceClassificationPushRecord<'a> {
    fn from(classification: &'a DeviceClassification) -> Self {
        DeviceClassificationPushRecord {
            id: Thing::from(&classification.id),
            common_name: &classification.common_name,
            extensions: classification.extensions.iter().map(Thing::from).collect(),
        }
    }
}

impl TryFrom<DeviceClassificationPullRecord> for DeviceClassification {
    type Error = anyhow::Error;
    fn try_from(classification: DeviceClassificationPullRecord) -> Result<Self, anyhow::Error> {
        Ok(DeviceClassification {
            id: DeviceClassificationID::try_from(classification.id)?,
            common_name: classification.common_name,
            extensions: classification
                .extensions
                .into_iter()
                .map(InventoryExtensionID::try_from)
                .collect::<Result<HashSet<_>, _>>()?,
        })
    }
}

impl<'a> From<&'a Device> for DevicePushRecord<'a> {
    fn from(device: &'a Device) -> Self {
        DevicePushRecord {
            id: Thing::from(&device.id),
            common_name: &device.common_name,
            manufacturer: Thing::from(&device.manufacturer),
            classification: Thing::from(&device.classification),
            extension: Thing::from(&device.extension),
            primary_model_identifiers: &device.primary_model_identifiers,
            extended_model_identifiers: &device.extended_model_identifiers,
        }
    }
}

impl TryFrom<DevicePullRecord> for Device {
    type Error = anyhow::Error;
    fn try_from(device: DevicePullRecord) -> Result<Self, Self::Error> {
        Ok(Device {
            id: DeviceID::try_from(device.id)?,
            common_name: device.common_name,
            manufacturer: DeviceManufacturerID::try_from(device.manufacturer)?,
            classification: DeviceClassificationID::try_from(device.classification)?,
            extension: InventoryExtensionID::try_from(device.extension)?,
            primary_model_identifiers: device.primary_model_identifiers,
            extended_model_identifiers: device.extended_model_identifiers,
        })
    }
}

impl From<&InventoryExtensionID> for Thing {
    fn from(id: &InventoryExtensionID) -> Self {
        Thing {
            tb: EXTENSION_TABLE_NAME.to_owned(),
            id: Id::String(id.to_non_namespaced_string()),
        }
    }
}

impl TryFrom<Thing> for InventoryExtensionID {
    type Error = anyhow::Error;
    fn try_from(thing: Thing) -> Result<Self, Self::Error> {
        if let Id::String(id) = thing.id {
            Ok(InventoryExtensionID::new(&id))
        } else {
            Err(anyhow!("Non-string ID for extension"))
        }
    }
}

impl From<&DeviceManufacturerID> for Thing {
    fn from(id: &DeviceManufacturerID) -> Self {
        Thing {
            tb: DEVICE_MANUFACTURER_TABLE_NAME.to_owned(),
            id: Id::String(id.to_non_namespaced_string()),
        }
    }
}

impl TryFrom<Thing> for DeviceManufacturerID {
    type Error = anyhow::Error;
    fn try_from(thing: Thing) -> Result<Self, Self::Error> {
        if let Id::String(id) = thing.id {
            Ok(DeviceManufacturerID::new(&id))
        } else {
            Err(anyhow!("Non-string ID for device manufacturer"))
        }
    }
}

impl From<&DeviceClassificationID> for Thing {
    fn from(id: &DeviceClassificationID) -> Self {
        Thing {
            tb: DEVICE_CLASSIFICATION_TABLE_NAME.to_owned(),
            id: Id::String(id.to_non_namespaced_string()),
        }
    }
}

impl TryFrom<Thing> for DeviceClassificationID {
    type Error = anyhow::Error;
    fn try_from(thing: Thing) -> Result<Self, Self::Error> {
        if let Id::String(id) = thing.id {
            Ok(DeviceClassificationID::new(&id))
        } else {
            Err(anyhow!("Non-string ID for device classification"))
        }
    }
}

impl From<&DeviceID> for Thing {
    fn from(id: &DeviceID) -> Self {
        Thing {
            tb: DEVICE_TABLE_NAME.to_owned(),
            id: Id::String(id.to_non_namespaced_string()),
        }
    }
}

impl TryFrom<Thing> for DeviceID {
    type Error = anyhow::Error;
    fn try_from(thing: Thing) -> Result<Self, Self::Error> {
        let id = match thing.id {
            Id::String(id) => id,
            _ => return Err(anyhow!("Non-string ID for device")),
        };

        let mut tokens = id.split('/');
        match (
            tokens.next(),
            tokens.next(),
            tokens.next(),
            tokens.next(),
            tokens.next(),
        ) {
            (
                Some(extension_id),
                Some(manufacturer_id),
                Some(classification_id),
                Some(id),
                None,
            ) => Ok(DeviceID::new(
                extension_id,
                manufacturer_id,
                classification_id,
                id,
            )),
            _ => Err(anyhow!("Improperly-formatted namespaced device ID")),
        }
    }
}
