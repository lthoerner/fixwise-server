use std::collections::HashSet;

use anyhow::anyhow;
use semver::Version;
use serde::{Deserialize, Serialize};
use surrealdb::sql::{Id, Thing};

use crate::database::{
    CLASSIFICATION_TABLE_NAME, DEVICE_TABLE_NAME, EXTENSION_TABLE_NAME, MANUFACTURER_TABLE_NAME,
};
use crate::extension::InventoryExtension;
use crate::models::common::{
    Classification, ClassificationID, Device, DeviceID, InventoryExtensionID,
    InventoryExtensionInfo, Manufacturer, ManufacturerID,
};

/// The metadata of an extension which can be added to the database.
#[derive(Debug, Serialize)]
pub struct InventoryExtensionInfoPushRecord<'a> {
    pub id: Thing,
    pub common_name: &'a str,
    pub version: String,
}

/// The metadata of an extension as read from the database.
#[derive(Debug, Deserialize)]
pub struct InventoryExtensionInfoPullRecord {
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
/// (particularly `Surreal.create()`) with type parameters when the actual result is not important.
#[derive(Debug, Deserialize)]
pub struct GenericPullRecord {
    // * This has to be an attribute tag because `_id` does not map to `id`.
    #[allow(dead_code)]
    id: Thing,
}

impl<'a> From<&'a InventoryExtensionInfo> for InventoryExtensionInfoPushRecord<'a> {
    fn from(extension: &'a InventoryExtensionInfo) -> Self {
        InventoryExtensionInfoPushRecord {
            id: Thing::from(&extension.id),
            common_name: &extension.common_name,
            version: extension.version.to_string(),
        }
    }
}

impl TryFrom<InventoryExtensionInfoPullRecord> for InventoryExtensionInfo {
    type Error = anyhow::Error;
    fn try_from(extension: InventoryExtensionInfoPullRecord) -> Result<Self, anyhow::Error> {
        Ok(InventoryExtensionInfo {
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

impl From<&InventoryExtension> for InventoryExtensionInfo {
    fn from(extension: &InventoryExtension) -> Self {
        InventoryExtensionInfo {
            id: extension.id.clone(),
            common_name: extension.name.clone(),
            version: extension.version.clone(),
        }
    }
}

impl From<&InventoryExtensionID> for Thing {
    fn from(id: &InventoryExtensionID) -> Self {
        Thing {
            tb: EXTENSION_TABLE_NAME.to_owned(),
            id: Id::String(id.non_namespaced_id.clone()),
        }
    }
}

impl TryFrom<Thing> for InventoryExtensionID {
    type Error = anyhow::Error;
    fn try_from(thing: Thing) -> Result<Self, Self::Error> {
        if let Id::String(id) = thing.id {
            Ok(InventoryExtensionID {
                non_namespaced_id: id,
            })
        } else {
            Err(anyhow!("Non-string ID for extension"))
        }
    }
}

impl From<&ManufacturerID> for Thing {
    fn from(id: &ManufacturerID) -> Self {
        Thing {
            tb: MANUFACTURER_TABLE_NAME.to_owned(),
            id: Id::String(id.non_namespaced_id.clone()),
        }
    }
}

impl TryFrom<Thing> for ManufacturerID {
    type Error = anyhow::Error;
    fn try_from(thing: Thing) -> Result<Self, Self::Error> {
        if let Id::String(id) = thing.id {
            Ok(ManufacturerID {
                non_namespaced_id: id,
            })
        } else {
            Err(anyhow!("Non-string ID for manufacturer"))
        }
    }
}

impl From<&ClassificationID> for Thing {
    fn from(id: &ClassificationID) -> Self {
        Thing {
            tb: CLASSIFICATION_TABLE_NAME.to_owned(),
            id: Id::String(id.non_namespaced_id.clone()),
        }
    }
}

impl TryFrom<Thing> for ClassificationID {
    type Error = anyhow::Error;
    fn try_from(thing: Thing) -> Result<Self, Self::Error> {
        if let Id::String(id) = thing.id {
            Ok(ClassificationID {
                non_namespaced_id: id,
            })
        } else {
            Err(anyhow!("Non-string ID for classification"))
        }
    }
}

impl From<&DeviceID> for Thing {
    fn from(id: &DeviceID) -> Self {
        Thing {
            tb: DEVICE_TABLE_NAME.to_owned(),
            id: Id::String(
                [
                    id.extension_id.to_non_namespaced_string().as_str(),
                    id.manufacturer_id.to_non_namespaced_string().as_str(),
                    id.classification_id.to_non_namespaced_string().as_str(),
                    id.non_namespaced_id.as_str(),
                ]
                .join("/"),
            ),
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
            ) => Ok(DeviceID {
                extension_id: InventoryExtensionID::new(extension_id),
                manufacturer_id: ManufacturerID::new(manufacturer_id),
                classification_id: ClassificationID::new(classification_id),
                non_namespaced_id: id.to_owned(),
            }),
            _ => Err(anyhow!("Improperly-formatted namespaced device ID")),
        }
    }
}
