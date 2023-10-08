use std::collections::HashSet;
use std::str::FromStr;
use std::{ffi::OsStr, path::Path};

use anyhow::anyhow;
use semver::Version;
use serde::Deserialize;

use crate::database::models::{
    Classification, ClassificationID, Device, DeviceID, ExtensionID, InventoryExtensionInfo,
    Manufacturer, ManufacturerID,
};
use crate::database::Database;

/// An extension of the database inventory system.
#[derive(Debug)]
pub struct InventoryExtension {
    pub id: ExtensionID,
    pub name: String,
    pub version: Version,
    pub load_override: bool,
    pub manufacturers: Vec<Manufacturer>,
    pub classifications: Vec<Classification>,
    pub devices: Vec<Device>,
}

/// An extension as read from a TOML file.
/// Some types are not compatible with the database, so this type must be converted into an
/// `Extension` before calling `Database::add_extension()`.
#[derive(Debug, Deserialize)]
struct ExtensionToml {
    extension_id: String,
    extension_common_name: String,
    extension_version: String,
    load_override: Option<bool>,
    manufacturers: Vec<ManufacturerToml>,
    classifications: Option<Vec<ClassificationToml>>,
    devices: Vec<DeviceToml>,
}

/// A device manufacturer as read from a TOML extension.
/// This must be converted into a `Manufacturer` before adding it to the database.
#[derive(Debug, Deserialize)]
struct ManufacturerToml {
    id: String,
    common_name: String,
}

/// A classification of device as read from a TOML extension.
/// This must be converted into a `Classification` before adding it to the database.
#[derive(Debug, Deserialize)]
struct ClassificationToml {
    id: String,
    common_name: String,
}

/// A device and its metadata as read from a TOML extension.
/// This must be converted into a `Device` before adding it to the database.
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
pub struct ExtensionManager {
    extensions: Vec<InventoryExtension>,
}

impl ExtensionManager {
    /// Loads all extensions from the default location (the extensions folder).
    // * Cannot use `Default` here due to error handling requirements.
    pub fn new() -> anyhow::Result<Self> {
        let extensions_dir_entries = std::fs::read_dir("./extensions")?;
        let mut extensions = Vec::new();
        for extension_file in extensions_dir_entries.flatten() {
            let (path, filetype) = (extension_file.path(), extension_file.file_type());
            if let Ok(filetype) = filetype {
                if filetype.is_file() && path.extension() == Some(OsStr::new("toml")) {
                    extensions.push(ExtensionManager::stage_extension(&path)?);
                }
            }
        }

        let extensions = extensions
            .into_iter()
            .map(InventoryExtension::from)
            .collect();

        Ok(ExtensionManager { extensions })
    }

    /// Parses a TOML file into an extension which can be added to the database by the manager.
    fn stage_extension(filename: &Path) -> anyhow::Result<ExtensionToml> {
        // ? Is it any better to read to bytes and convert to struct or is string fine?
        let toml = std::fs::read_to_string(filename)?;
        let extension: ExtensionToml = toml::from_str(&toml)?;
        Ok(extension)
    }

    /// Adds all extensions from the manager into the database, handling any conflicts.
    // ? How will callbacks be handled here? Probably need to do some sort of DI pattern.
    pub async fn load_extensions(self, db: &Database) -> anyhow::Result<()> {
        let loaded_extensions = db.list_extensions().await?;
        'staged_extension: for staged_extension in self.extensions.into_iter() {
            let staged_extension_info = InventoryExtensionInfo::from(&staged_extension);
            for loaded_extension_info in &loaded_extensions {
                if staged_extension_info == *loaded_extension_info {
                    if !staged_extension.load_override {
                        continue 'staged_extension;
                    } else {
                        // * Though it is theoretically possible that another extension may run
                        // * into a similar conflict with a different outcome, it should never be
                        // * the case that two extensions with the same ID exist in the database.
                        db.unload_extension(loaded_extension_info).await?;
                    }
                } else if staged_extension_info.id == loaded_extension_info.id
                    && staged_extension_info.common_name != loaded_extension_info.common_name
                    && staged_extension_info.version == loaded_extension_info.version
                {
                    return Err(anyhow!(
                        "Extension '{0}' has ID '{1}' but '{1}' is already loaded",
                        staged_extension_info.common_name,
                        staged_extension_info.id.to_non_namespaced_string(),
                    ));
                } else if staged_extension_info.id == loaded_extension_info.id
                    && staged_extension_info.version != loaded_extension_info.version
                {
                    // * If the ID is the same, but the version is different, the plugin can be
                    // * updated or downgraded. Upgrades will happen automatically, but the user
                    // * must be prompted for a downgrade to occur.
                    // * The extension name can change between versions, so it is not checked.
                    // TODO: Add prompt for downgrade
                    db.unload_extension(loaded_extension_info).await?;
                }
            }

            db.load_extension(staged_extension).await?;
        }

        // TODO: Add checks for duplicate manufacturers and classifications

        Ok(())
    }
}

// TODO: Remove unwraps
// * Inner types here (`Manufacturer`, `Classification`, `Device`) must be converted with context
// * provided by the `ExtensionToml` itself, so they cannot be converted directly.
impl From<ExtensionToml> for InventoryExtension {
    fn from(toml: ExtensionToml) -> Self {
        let manufacturers = toml
            .manufacturers
            .into_iter()
            .map(|m| Manufacturer {
                id: ManufacturerID::new(m.id),
                common_name: m.common_name,
                extensions: HashSet::from([ExtensionID::new(toml.extension_id.clone())]),
            })
            .collect();

        let classifications = toml
            .classifications
            .unwrap_or_default()
            .into_iter()
            .map(|c| Classification {
                id: ClassificationID::new(c.id),
                common_name: c.common_name,
                extensions: HashSet::from([ExtensionID::new(toml.extension_id.clone())]),
            })
            .collect();

        let devices = toml
            .devices
            .into_iter()
            // ? Is there a more conventional way to do this conversion?
            .map(|d| Device {
                id: DeviceID::new(
                    toml.extension_id.clone(),
                    d.manufacturer.clone(),
                    d.classification.clone(),
                    d.true_name.clone(),
                ),
                common_name: d.common_name,
                manufacturer: ManufacturerID::new(d.manufacturer.clone()),
                classification: ClassificationID::new(d.classification.clone()),
                extension: ExtensionID::new(toml.extension_id.clone()),
                primary_model_identifiers: d.primary_model_identifiers,
                extended_model_identifiers: d.extended_model_identifiers,
            })
            .collect();

        InventoryExtension {
            id: ExtensionID::new(toml.extension_id.clone()),
            name: toml.extension_common_name,
            version: Version::from_str(&toml.extension_version).unwrap(),
            load_override: toml.load_override.unwrap_or_default(),
            manufacturers,
            classifications,
            devices,
        }
    }
}