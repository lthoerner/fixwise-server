use std::collections::HashSet;
use std::ffi::OsStr;
use std::fs::DirEntry;
use std::path::Path;
use std::str::FromStr;

use anyhow::anyhow;
use log::{error, info, warn};
use semver::Version;
use serde::Deserialize;

use crate::database::Database;
use crate::models::common::{
    Classification, ClassificationID, Device, DeviceID, InventoryExtensionID,
    InventoryExtensionInfo, Manufacturer, ManufacturerID,
};

/// An extension of the database inventory system.
#[derive(Debug, Clone)]
pub struct InventoryExtension {
    pub id: InventoryExtensionID,
    pub name: String,
    pub version: Version,
    pub load_override: bool,
    pub manufacturers: Vec<Manufacturer>,
    pub classifications: Vec<Classification>,
    pub devices: Vec<Device>,
}

/// An inventory extension as read from a TOML file.
/// Some types are not compatible with the database, so this type must be converted into an
/// `InventoryExtension` before calling `Database::load_extension()`.
#[derive(Debug, Deserialize)]
struct InventoryExtensionToml {
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
#[derive(Default)]
pub struct ExtensionManager {
    extensions: Vec<InventoryExtension>,
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
                info!("Staging extension...");
                manager.stage_extension(&extension_file.path())?;
            }
        }

        Ok(manager)
    }

    /// Creates a manager for the provided extensions.
    #[cfg(test)]
    #[allow(dead_code)]
    pub fn with_extensions(extensions: impl IntoIterator<Item = InventoryExtension>) -> Self {
        Self {
            extensions: extensions.into_iter().collect(),
        }
    }

    /// Parses a TOML file into an extension which can be added to the database by the manager.
    fn stage_extension(&mut self, filename: &Path) -> anyhow::Result<()> {
        // ? Is it any better to read to bytes and convert to struct or is string fine?
        let toml = std::fs::read_to_string(filename)?;
        let extension: InventoryExtensionToml = toml::from_str(&toml)?;
        self.extensions.push(InventoryExtension::from(extension));

        Ok(())
    }

    /// Adds all extensions from the manager into the database, handling any conflicts.
    // ? How will callbacks be handled here? Probably need to do some sort of DI pattern.
    pub async fn load_extensions(self, db: &Database) -> anyhow::Result<()> {
        info!("Loading staged extensions into database...");
        let loaded_extensions = db.list_extensions().await?;
        'staged_extension: for staged_extension in self.extensions.into_iter() {
            let staged_extension_info = InventoryExtensionInfo::from(&staged_extension);
            for loaded_extension_info in &loaded_extensions {
                if staged_extension_info == *loaded_extension_info {
                    if !staged_extension.load_override {
                        info!(
                            "Skipped extension '{}' because it is already loaded.",
                            staged_extension_info.common_name
                        );
                        continue 'staged_extension;
                    } else {
                        // * Though it is theoretically possible that another extension may run
                        // * into a similar conflict with a different outcome, it should never be
                        // * the case that two extensions with the same ID exist in the database.
                        warn!(
                            "Reloading extension '{}' due to a load override.",
                            loaded_extension_info.common_name
                        );
                        db.unload_extension(loaded_extension_info).await?;
                    }
                // TODO: The two below conditions are incompatible, need to fix
                } else if staged_extension_info.id == loaded_extension_info.id
                    && staged_extension_info.common_name != loaded_extension_info.common_name
                    && staged_extension_info.version == loaded_extension_info.version
                {
                    error!(
                        "Staged extension '{0}' and loaded extension '{1}' both have ID '{2}'.",
                        staged_extension_info.common_name,
                        loaded_extension_info.common_name,
                        staged_extension_info.id.to_non_namespaced_string()
                    );
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
                    // TODO: Add user prompt for downgrade
                    if staged_extension_info.version < loaded_extension_info.version {
                        warn!(
                            "Downgrading loaded extension '{}' from version {} to {}.",
                            staged_extension_info.common_name,
                            loaded_extension_info.version,
                            staged_extension_info.version
                        );
                    } else {
                        info!(
                            "Upgrading loaded extension '{}' from version {} to {}.",
                            staged_extension_info.common_name,
                            loaded_extension_info.version,
                            staged_extension_info.version
                        );
                    }

                    db.unload_extension(loaded_extension_info).await?;
                }
            }

            info!(
                "Loading extension '{}' into database...",
                staged_extension_info.common_name
            );
            db.load_extension(staged_extension).await?;
            info!("Extension loaded.")
        }

        // TODO: Add checks for duplicate manufacturers and classifications

        Ok(())
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

impl InventoryExtension {
    /// Creates a basic extension for testing purposes.
    /// Can be modified to test different scenarios.
    #[cfg(test)]
    #[allow(dead_code)]
    pub fn test(num: u32) -> Self {
        Self {
            id: InventoryExtensionID::new(&format!("test_{num}")),
            name: format!("Test Extension {num}"),
            version: Version::new(1, 0, 0),
            load_override: false,
            manufacturers: Vec::new(),
            classifications: Vec::new(),
            devices: Vec::new(),
        }
    }
}

// TODO: Remove unwraps
// * Inner types here (`Manufacturer`, `Classification`, `Device`) must be converted with context
// * provided by the `ExtensionToml` itself, so they cannot be converted directly.
impl From<InventoryExtensionToml> for InventoryExtension {
    fn from(toml: InventoryExtensionToml) -> Self {
        let manufacturers = toml
            .manufacturers
            .into_iter()
            .map(|m| Manufacturer {
                id: ManufacturerID::new(&m.id),
                common_name: m.common_name,
                extensions: HashSet::from([InventoryExtensionID::new(&toml.extension_id)]),
            })
            .collect();

        let classifications = toml
            .classifications
            .unwrap_or_default()
            .into_iter()
            .map(|c| Classification {
                id: ClassificationID::new(&c.id),
                common_name: c.common_name,
                extensions: HashSet::from([InventoryExtensionID::new(&toml.extension_id)]),
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
                manufacturer: ManufacturerID::new(&d.manufacturer),
                classification: ClassificationID::new(&d.classification),
                extension: InventoryExtensionID::new(&toml.extension_id),
                primary_model_identifiers: d.primary_model_identifiers,
                extended_model_identifiers: d.extended_model_identifiers,
            })
            .collect();

        InventoryExtension {
            id: InventoryExtensionID::new(&toml.extension_id),
            name: toml.extension_common_name,
            version: Version::from_str(&toml.extension_version).unwrap(),
            load_override: toml.load_override.unwrap_or_default(),
            manufacturers,
            classifications,
            devices,
        }
    }
}

#[cfg(test)]
mod tests {
    use semver::Version;

    use super::{ExtensionManager, InventoryExtension};
    use crate::database::Database;
    use crate::models::common::{
        Classification, Device, InventoryExtensionID, InventoryExtensionInfo, Manufacturer,
    };

    #[tokio::test]
    #[ignore = "not implemented"]
    /// Tests that two extensions with the same ID, but incompatible metadata, will cause an error.
    async fn incompatible_duplicate_extensions() {
        let db = Database::connect_with_name("test_incompatible_duplicate_extensions").await;
        db.teardown().await;
        todo!()
    }

    #[tokio::test]
    /// Tests that two extensions with the same ID and metadata will not be reloaded or cause a
    /// conflict, even if they have different contents.
    async fn compatible_duplicate_extensions() {
        let db = Database::connect_with_name("test_compatible_duplicate_extensions").await;

        // Create two extensions with the same ID and metadata, but different contents
        let mut original_extension = InventoryExtension::test(1);
        let mut duplicate_extension = original_extension.clone();
        // Add a different manufacturer to each extension
        let manufacturer_1 = Manufacturer::test(1, &original_extension.id);
        let manufacturer_2 = Manufacturer::test(2, &duplicate_extension.id);
        original_extension
            .manufacturers
            .push(manufacturer_1.clone());
        duplicate_extension
            .manufacturers
            .push(manufacturer_2.clone());
        // Add a different classification to each extension
        let classification_1 = Classification::test(1, &original_extension.id);
        let classification_2 = Classification::test(2, &duplicate_extension.id);
        original_extension
            .classifications
            .push(classification_1.clone());
        duplicate_extension
            .classifications
            .push(classification_2.clone());
        // Add a different device to each extension
        let device_1 = Device::test(
            1,
            &original_extension.id,
            &manufacturer_1.id,
            &classification_1.id,
        );
        let device_2 = Device::test(
            2,
            &duplicate_extension.id,
            &manufacturer_2.id,
            &classification_2.id,
        );
        original_extension.devices.push(device_1.clone());
        duplicate_extension.devices.push(device_2.clone());

        // Load the first extension into the database
        let manager = ExtensionManager::with_extensions([original_extension.clone()]);
        manager.load_extensions(&db).await.unwrap();
        // Make sure the extension was loaded correctly
        assert!(db.only_contains(&original_extension).await.unwrap());
        // Load the second extension into the database
        let manager = ExtensionManager::with_extensions([duplicate_extension.clone()]);
        manager.load_extensions(&db).await.unwrap();
        // Make sure the second extension was not loaded
        assert!(db.only_contains(&original_extension).await.unwrap());

        db.teardown().await;
    }

    #[tokio::test]
    /// Tests that an extension will be replaced by an updated version of itself.
    async fn reload_extension_update() {
        let db = Database::connect_with_name("test_reload_extension_update").await;

        // Create two extensions with the same ID, but different versions
        let mut original_extension = InventoryExtension::test(1);
        let mut updated_extension = InventoryExtension::test(1);
        updated_extension.version = Version::new(1, 0, 1);
        // Add a different manufacturer to each extension
        let manufacturer_1 = Manufacturer::test(1, &original_extension.id);
        let manufacturer_2 = Manufacturer::test(2, &updated_extension.id);
        original_extension
            .manufacturers
            .push(manufacturer_1.clone());
        updated_extension.manufacturers.push(manufacturer_2.clone());
        // Add a different classification to each extension
        let classification_1 = Classification::test(1, &original_extension.id);
        let classification_2 = Classification::test(2, &updated_extension.id);
        original_extension
            .classifications
            .push(classification_1.clone());
        updated_extension
            .classifications
            .push(classification_2.clone());
        // Add a different device to each extension
        let device_1 = Device::test(
            1,
            &original_extension.id,
            &manufacturer_1.id,
            &classification_1.id,
        );
        let device_2 = Device::test(
            2,
            &updated_extension.id,
            &manufacturer_2.id,
            &classification_2.id,
        );
        original_extension.devices.push(device_1.clone());
        updated_extension.devices.push(device_2.clone());

        // Load the first extension into the database
        let manager = ExtensionManager::with_extensions([original_extension.clone()]);
        manager.load_extensions(&db).await.unwrap();
        // Make sure the extension was loaded correctly
        assert!(db.only_contains(&original_extension).await.unwrap());
        // Reload the extension with the updated version, which should unload the original extension
        let manager = ExtensionManager::with_extensions([updated_extension.clone()]);
        manager.load_extensions(&db).await.unwrap();
        // Make sure the original extension was unloaded and the new version was loaded
        assert!(db.only_contains(&updated_extension).await.unwrap());

        db.teardown().await;
    }

    #[tokio::test]
    /// Tests that an extension will be replaced by the same extension with the load override flag.
    async fn reload_extension_override() {
        let db = Database::connect_with_name("test_reload_extension_override").await;

        // Create two extensions with the same metadata, but with a load override
        let mut original_extension = InventoryExtension::test(1);
        let mut reloaded_extension = InventoryExtension::test(1);
        reloaded_extension.load_override = true;
        // Add a different manufacturer to each extension
        let manufacturer_1 = Manufacturer::test(1, &original_extension.id);
        let manufacturer_2 = Manufacturer::test(2, &reloaded_extension.id);
        original_extension
            .manufacturers
            .push(manufacturer_1.clone());
        reloaded_extension
            .manufacturers
            .push(manufacturer_2.clone());
        // Add a different classification to each extension
        let classification_1 = Classification::test(1, &original_extension.id);
        let classification_2 = Classification::test(2, &reloaded_extension.id);
        original_extension
            .classifications
            .push(classification_1.clone());
        reloaded_extension
            .classifications
            .push(classification_2.clone());
        // Add a different device to each extension
        let device_1 = Device::test(
            1,
            &original_extension.id,
            &manufacturer_1.id,
            &classification_1.id,
        );
        let device_2 = Device::test(
            2,
            &reloaded_extension.id,
            &manufacturer_2.id,
            &classification_2.id,
        );
        original_extension.devices.push(device_1.clone());
        reloaded_extension.devices.push(device_2.clone());

        // Load the first extension into the database
        let manager = ExtensionManager::with_extensions([original_extension.clone()]);
        manager.load_extensions(&db).await.unwrap();
        // Make sure the extension was loaded correctly
        assert!(db.only_contains(&original_extension).await.unwrap());
        // Reload the extension, which should unload the original extension
        let manager = ExtensionManager::with_extensions([reloaded_extension.clone()]);
        manager.load_extensions(&db).await.unwrap();
        // Make sure the original extension was unloaded and the new version was loaded
        assert!(db.only_contains(&reloaded_extension).await.unwrap());

        db.teardown().await;
    }

    #[tokio::test]
    async fn unload_builtin_extension() {
        let db = Database::connect_with_name("test_unload_builtin_extension").await;
        db.setup_tables().await.unwrap();
        db.setup_reserved_items().await.unwrap();

        // TODO: Match on error variant once custom errors are added
        assert!(db
            .unload_extension(&InventoryExtensionInfo {
                id: InventoryExtensionID::new("builtin"),
                common_name: "Built-in".to_owned(),
                version: Version::new(0, 0, 0)
            })
            .await
            .is_err());

        db.teardown().await;
    }
}
