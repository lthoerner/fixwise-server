mod conflicts;
#[cfg(test)]
mod tests;

use std::collections::HashSet;
use std::ffi::OsStr;
use std::fs::DirEntry;
use std::path::Path;
use std::str::FromStr;

use log::{error, info};
use semver::Version;
use serde::Deserialize;

use self::conflicts::{LoadConflict, StageConflict};
use crate::database::Database;
use crate::models::common::ids::{
    DeviceClassificationUniqueID, DeviceID, DeviceManufacturerUniqueID,
    InventoryExtensionUniqueID as ExtensionID, UniqueID,
};
use crate::models::common::{
    Device, DeviceClassification, DeviceManufacturer, InventoryExtensionMetadata as Metadata,
};

/// An extension of the database inventory system.
#[derive(Debug, Clone)]
pub struct InventoryExtension {
    pub metadata: Metadata,
    pub device_manufacturers: Vec<DeviceManufacturer>,
    pub device_classifications: Vec<DeviceClassification>,
    pub devices: Vec<Device>,
}

/// An inventory extension as read from a TOML file.
/// Some types are not compatible with the database, so this type must be converted into an
/// [`InventoryExtension`] before calling [`Database::load_extension`].
#[derive(Debug, Deserialize)]
struct InventoryExtensionToml {
    extension_id: String,
    extension_common_name: String,
    extension_version: String,
    device_manufacturers: Option<Vec<DeviceManufacturerToml>>,
    device_classifications: Option<Vec<DeviceClassificationToml>>,
    devices: Vec<DeviceToml>,
}

/// A device manufacturer as read from a TOML extension.
/// This must be converted into a [`DeviceManufacturer`] before adding it to the database.
#[derive(Debug, Deserialize)]
struct DeviceManufacturerToml {
    id: String,
    common_name: String,
}

/// A classification of device as read from a TOML extension.
/// This must be converted into a [`DeviceClassification`] before adding it to the database.
#[derive(Debug, Deserialize)]
struct DeviceClassificationToml {
    id: String,
    common_name: String,
}

/// A device and its metadata as read from a TOML extension.
/// This must be converted into a [`Device`] before adding it to the database.
#[derive(Debug, Deserialize)]
pub struct DeviceToml {
    // TODO: Figure out a better name for this
    true_name: String,
    common_name: String,
    manufacturer: String,
    classification: String,
    primary_model_identifiers: Vec<String>,
    extended_model_identifiers: Vec<String>,
}

/// Manages the parsing and loading of extensions into the database.
#[derive(Default)]
pub struct ExtensionManager {
    staged_extensions: Vec<InventoryExtension>,
}

impl ExtensionManager {
    /// Loads all extensions from the default location (the extensions folder).
    pub fn new() -> anyhow::Result<Self> {
        let mut manager = Self::default();
        for extension_file in std::fs::read_dir("./extensions")?.flatten() {
            if Self::is_extension(&extension_file) {
                info!(
                    "Located extension file: {}",
                    extension_file.path().display()
                );
                manager.stage_extension(Self::parse_extension(&extension_file.path())?)?;
            }
        }

        Ok(manager)
    }

    /// Parses a TOML file into an extensoin which can be added to the database by the manager.
    fn parse_extension(filename: &Path) -> anyhow::Result<InventoryExtension> {
        let toml = std::fs::read_to_string(filename)?;
        let extension_toml: InventoryExtensionToml = toml::from_str(&toml)?;
        Ok(InventoryExtension::from(extension_toml))
    }

    /// Stages an extension, checking whether it conflicts with other already-staged extensions.
    fn stage_extension(
        &mut self,
        extension: InventoryExtension,
    ) -> anyhow::Result<Option<StageConflict>> {
        if !self.already_contains(&extension) {
            info!(
                "Staging extension '{}'.",
                extension.metadata.id.unnamespaced()
            );
            self.staged_extensions.push(extension);
        } else {
            // $ NOTIFICATION OR PROMPT HERE
            error!(
                "Extension with ID '{}' already staged, skipping.",
                extension.metadata.id.unnamespaced()
            );
            return Ok(Some(StageConflict::new(&extension.metadata)));
        }

        Ok(None)
    }

    /// Checks whether a given extension shares an ID with any of the already-staged extensions.
    fn already_contains(&self, extension: &InventoryExtension) -> bool {
        let extension_id = &extension.metadata.id;
        for staged_extension in &self.staged_extensions {
            let staged_extension_id = &staged_extension.metadata.id;
            if extension_id == staged_extension_id {
                return true;
            }
        }

        false
    }

    /// Adds all extensions from the manager into the database, handling any conflicts.
    // ? How will callbacks be handled here? Probably need to do some sort of DI pattern.
    pub async fn load_extensions(
        self,
        db: &Database,
        load_override: bool,
    ) -> anyhow::Result<Vec<LoadConflict>> {
        info!("Loading staged extensions into database...");
        let mut loaded_extensions = db.list_extensions().await?;

        let mut conflicts = Vec::new();
        'current_extension: for staged_extension in self.staged_extensions.into_iter() {
            let staged_extension_metadata = &staged_extension.metadata;
            let staged_extension_id = staged_extension_metadata.id.unnamespaced();

            let Some(conflict) = LoadConflict::new(&staged_extension, &mut loaded_extensions)
            else {
                info!("Loading extension '{}'.", &staged_extension_id);
                db.load_extension(staged_extension).await?;
                continue 'current_extension;
            };

            conflict.log(load_override);
            if load_override || conflict.should_reload() {
                db.reload_extension(staged_extension).await?;
            }

            conflicts.push(conflict);
        }

        Ok(conflicts)
    }

    /// Checks whether a given filesystem object is a valid extension.
    fn is_extension(object: &DirEntry) -> bool {
        let (path, filetype) = (object.path(), object.file_type());
        if let Ok(filetype) = filetype {
            if filetype.is_file() && path.extension() == Some(OsStr::new("toml")) {
                return true;
            }
        }

        false
    }
}

// TODO: Remove unwraps
// * Inner types here ([`DeviceManufacturer`], [`DeviceClassification`], [`Device`]) must be
// * converted with context provided by the [`ExtensionToml`] itself, so they cannot be converted
// * directly.
impl From<InventoryExtensionToml> for InventoryExtension {
    fn from(toml: InventoryExtensionToml) -> Self {
        let device_manufacturers = toml
            .device_manufacturers
            .unwrap_or_default()
            .into_iter()
            .map(|m| DeviceManufacturer {
                id: DeviceManufacturerUniqueID::new(&m.id),
                common_name: m.common_name,
                extensions: HashSet::from([ExtensionID::new(&toml.extension_id)]),
            })
            .collect();

        let device_classifications = toml
            .device_classifications
            .unwrap_or_default()
            .into_iter()
            .map(|c| DeviceClassification {
                id: DeviceClassificationUniqueID::new(&c.id),
                common_name: c.common_name,
                extensions: HashSet::from([ExtensionID::new(&toml.extension_id)]),
            })
            .collect();

        let devices = toml
            .devices
            .into_iter()
            // ? Is there a more conventional way to do this conversion?
            .map(|d| Device {
                id: DeviceID::new(
                    &toml.extension_id,
                    &d.manufacturer,
                    &d.classification,
                    &d.true_name,
                ),
                common_name: d.common_name,
                manufacturer: DeviceManufacturerUniqueID::new(&d.manufacturer),
                classification: DeviceClassificationUniqueID::new(&d.classification),
                extension: ExtensionID::new(&toml.extension_id),
                primary_model_identifiers: d.primary_model_identifiers,
                extended_model_identifiers: d.extended_model_identifiers,
            })
            .collect();

        InventoryExtension {
            metadata: Metadata {
                id: ExtensionID::new(&toml.extension_id),
                common_name: toml.extension_common_name,
                version: Version::from_str(&toml.extension_version).unwrap(),
            },
            device_manufacturers,
            device_classifications,
            devices,
        }
    }
}
