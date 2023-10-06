use std::str::FromStr;
use std::{ffi::OsStr, path::Path};

use semver::Version;
use serde::Deserialize;

use crate::database::models::{
    Classification, ClassificationID, Device, DeviceID, ExtensionID, Manufacturer, ManufacturerID,
};
use crate::database::Database;

/// An extension of the database inventory system.
#[derive(Debug)]
pub struct InventoryExtension {
    pub id: ExtensionID,
    pub name: String,
    pub version: Version,
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

impl DeviceToml {
    /// Generates a namespaced ID for the device, allowing devices of different extensions,
    /// manufacturers, or classifications to share the same name.
    pub fn generate_id(&self, extension_name: &str) -> String {
        [
            extension_name,
            &self.manufacturer,
            &self.classification,
            &self.true_name,
        ]
        .join("/")
    }
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
                    let extension = ExtensionManager::load_extension(&path)?;
                    if ExtensionManager::validate(&extension, &extensions) {
                        extensions.push(extension);
                    }
                }
            }
        }

        let extensions = extensions
            .into_iter()
            .map(InventoryExtension::from)
            .collect();

        Ok(ExtensionManager { extensions })
    }

    /// Parses a TOML file into an extension which can be added to the database.
    fn load_extension(filename: &Path) -> anyhow::Result<ExtensionToml> {
        // ? Is it any better to read to bytes and convert to struct or is string fine?
        let toml = std::fs::read_to_string(filename)?;
        let extension: ExtensionToml = toml::from_str(&toml)?;
        Ok(extension)
    }

    /// Adds all extensions from the manager into the database, handling any conflicts.
    // ? How will callbacks be handled here?
    pub async fn add_extensions(&self, db: &Database) -> anyhow::Result<()> {
        todo!()
    }

    /// Validates that the given extension does not have elements which conflict with another
    /// extension which has already been loaded.
    // TODO: Make this a `Result` return to define different conflicts probably
    fn validate(extension_to_load: &ExtensionToml, loaded_extensions: &[ExtensionToml]) -> bool {
        // TODO: Add other checks
        for loaded_extension in loaded_extensions {
            if loaded_extension.extension_common_name == extension_to_load.extension_common_name {
                return false;
            }
        }

        true
    }
}

// TODO: Remove unwraps
impl From<ExtensionToml> for InventoryExtension {
    fn from(toml: ExtensionToml) -> Self {
        let extension_name = toml.extension_common_name;
        let devices = toml
            .devices
            .into_iter()
            // ? Is there a more conventional way to do this conversion?
            .map(|d| Device {
                id: DeviceID::new(
                    extension_name.clone(),
                    d.manufacturer.clone(),
                    d.classification.clone(),
                    d.true_name.clone(),
                ),
                common_name: d.common_name,
                manufacturer: ManufacturerID::new(d.manufacturer.clone()),
                classification: ClassificationID::new(d.classification.clone()),
                extension: ExtensionID::new(extension_name.clone()),
                primary_model_identifiers: d.primary_model_identifiers,
                extended_model_identifiers: d.extended_model_identifiers,
            })
            .collect();

        InventoryExtension {
            id: ExtensionID::new(toml.extension_id),
            name: extension_name,
            version: Version::from_str(&toml.extension_version).unwrap(),
            manufacturers: toml
                .manufacturers
                .into_iter()
                .map(Manufacturer::from)
                .collect(),
            classifications: toml
                .classifications
                .unwrap_or_default()
                .into_iter()
                .map(Classification::from)
                .collect(),
            devices,
        }
    }
}

impl From<ManufacturerToml> for Manufacturer {
    fn from(toml: ManufacturerToml) -> Self {
        Manufacturer {
            id: ManufacturerID::new(toml.id),
            common_name: toml.common_name,
        }
    }
}

impl From<ClassificationToml> for Classification {
    fn from(toml: ClassificationToml) -> Self {
        Classification {
            id: ClassificationID::new(toml.id),
            common_name: toml.common_name,
        }
    }
}
