use std::collections::HashSet;

use anyhow::anyhow;
use semver::Version;
use surrealdb::sql::{Id, Thing};

use super::common::{
    Classification, ClassificationID, Device, DeviceID, InventoryExtensionID,
    InventoryExtensionMetadata, Manufacturer, ManufacturerID,
};
use super::database::{
    ClassificationPullRecord, ClassificationPushRecord, DevicePullRecord, DevicePushRecord,
    InventoryExtensionMetadataPullRecord, InventoryExtensionMetadataPushRecord,
    ManufacturerPullRecord, ManufacturerPushRecord,
};
use crate::database::{
    CLASSIFICATION_TABLE_NAME, DEVICE_TABLE_NAME, EXTENSION_TABLE_NAME, MANUFACTURER_TABLE_NAME,
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

impl<'a> From<&'a Manufacturer> for ManufacturerPushRecord<'a> {
    fn from(manufacturer: &'a Manufacturer) -> Self {
        ManufacturerPushRecord {
            id: Thing::from(&manufacturer.id),
            common_name: &manufacturer.common_name,
            extensions: manufacturer.extensions.iter().map(Thing::from).collect(),
        }
    }
}

impl TryFrom<ManufacturerPullRecord> for Manufacturer {
    type Error = anyhow::Error;
    fn try_from(manufacturer: ManufacturerPullRecord) -> Result<Self, anyhow::Error> {
        Ok(Manufacturer {
            id: ManufacturerID::try_from(manufacturer.id)?,
            common_name: manufacturer.common_name,
            extensions: manufacturer
                .extensions
                .into_iter()
                .map(InventoryExtensionID::try_from)
                .collect::<Result<HashSet<_>, _>>()?,
        })
    }
}

impl<'a> From<&'a Classification> for ClassificationPushRecord<'a> {
    fn from(classification: &'a Classification) -> Self {
        ClassificationPushRecord {
            id: Thing::from(&classification.id),
            common_name: &classification.common_name,
            extensions: classification.extensions.iter().map(Thing::from).collect(),
        }
    }
}

impl TryFrom<ClassificationPullRecord> for Classification {
    type Error = anyhow::Error;
    fn try_from(classification: ClassificationPullRecord) -> Result<Self, anyhow::Error> {
        Ok(Classification {
            id: ClassificationID::try_from(classification.id)?,
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
            manufacturer: ManufacturerID::try_from(device.manufacturer)?,
            classification: ClassificationID::try_from(device.classification)?,
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

impl From<&ManufacturerID> for Thing {
    fn from(id: &ManufacturerID) -> Self {
        Thing {
            tb: MANUFACTURER_TABLE_NAME.to_owned(),
            id: Id::String(id.to_non_namespaced_string()),
        }
    }
}

impl TryFrom<Thing> for ManufacturerID {
    type Error = anyhow::Error;
    fn try_from(thing: Thing) -> Result<Self, Self::Error> {
        if let Id::String(id) = thing.id {
            Ok(ManufacturerID::new(&id))
        } else {
            Err(anyhow!("Non-string ID for manufacturer"))
        }
    }
}

impl From<&ClassificationID> for Thing {
    fn from(id: &ClassificationID) -> Self {
        Thing {
            tb: CLASSIFICATION_TABLE_NAME.to_owned(),
            id: Id::String(id.to_non_namespaced_string()),
        }
    }
}

impl TryFrom<Thing> for ClassificationID {
    type Error = anyhow::Error;
    fn try_from(thing: Thing) -> Result<Self, Self::Error> {
        if let Id::String(id) = thing.id {
            Ok(ClassificationID::new(&id))
        } else {
            Err(anyhow!("Non-string ID for classification"))
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
