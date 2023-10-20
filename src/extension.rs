use std::collections::HashSet;
use std::ffi::OsStr;
use std::fs::DirEntry;
use std::path::Path;
use std::str::FromStr;

use semver::Version;
use serde::Deserialize;
use tracing::{error, info, warn};

use crate::database::Database;
use crate::models::common::{
    Classification, ClassificationID, Device, DeviceID, InventoryExtensionID,
    InventoryExtensionMetadata, Manufacturer, ManufacturerID,
};

/// An extension of the database inventory system.
#[derive(Debug, Clone)]
pub struct InventoryExtension {
    pub metadata: InventoryExtensionMetadata,
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
                manager.stage_extension(&extension_file.path())?;
            }
        }

        Ok(manager)
    }

    /// Creates a manager for the provided extensions.
    #[cfg(test)]
    #[allow(dead_code)]
    pub fn with_extensions(extensions: impl IntoIterator<Item = InventoryExtension>) -> Self {
        let mut manager = Self::default();
        for extension in extensions {
            manager.staged_extensions.push(extension);
        }

        manager
    }

    /// Parses a TOML file into an extension which can be added to the database by the manager.
    fn stage_extension(&mut self, filename: &Path) -> anyhow::Result<()> {
        let toml = std::fs::read_to_string(filename)?;
        let extension_toml: InventoryExtensionToml = toml::from_str(&toml)?;
        let extension = InventoryExtension::from(extension_toml);
        if !self.already_contains(&extension) {
            info!(
                "Extension '{}' staged.",
                extension.metadata.id.to_non_namespaced_string()
            );
            self.staged_extensions.push(extension);
        } else {
            // $ NOTIFICATION OR PROMPT HERE
            error!(
                "Extension with ID '{}' already staged, skipping.",
                extension.metadata.id.to_non_namespaced_string()
            );
        }

        Ok(())
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
    pub async fn load_extensions(self, db: &Database) -> anyhow::Result<()> {
        info!("Loading staged extensions into database...");
        let loaded_extensions = db.list_extensions().await?;

        'current_extension: for staged_extension in self.staged_extensions.into_iter() {
            let staged_extension_metadata = &staged_extension.metadata;
            let staged_extension_id = staged_extension_metadata.id.to_non_namespaced_string();
            'conflict_check: for loaded_extension_metadata in &loaded_extensions {
                // * Two extensions are considered to be the same if they have all the same
                // * metadata, including the ID, name, and version, regardless of their contents.
                let same_extension = staged_extension_metadata == loaded_extension_metadata;

                let same_id =
                    same_extension || staged_extension_metadata.id == loaded_extension_metadata.id;

                if !same_id {
                    continue 'conflict_check;
                }

                let same_name = same_extension
                    || staged_extension_metadata.common_name
                        == loaded_extension_metadata.common_name;
                let different_name = !same_name;

                let updating = same_id
                    && staged_extension_metadata.version > loaded_extension_metadata.version;
                let downgrading = same_id
                    && staged_extension_metadata.version < loaded_extension_metadata.version;

                if different_name {
                    if updating || downgrading {
                        // $ NOTIFICATION HERE
                        warn!(
                            "Loaded extension '{}' has the name '{}', but a staged extension with \
                            the same ID, but a different version, has the name '{}'.",
                            &staged_extension_id,
                            loaded_extension_metadata.common_name,
                            staged_extension_metadata.common_name
                        )
                    } else {
                        // $ PROMPT HERE
                        error!(
                            "Loaded extension '{}' has the name '{}', but a staged extension with \
                            the same ID and version has the name '{}', so it has been skipped.",
                            &staged_extension_id,
                            loaded_extension_metadata.common_name,
                            staged_extension_metadata.common_name
                        );
                        continue 'current_extension;
                    }
                }

                if same_extension {
                    if staged_extension.load_override {
                        warn!(
                            "Reloading extension '{}' due to a load override.",
                            &staged_extension_id
                        );
                        db.unload_extension(&loaded_extension_metadata.id).await?;
                        break 'conflict_check;
                    } else {
                        info!(
                            "Skipped extension '{}' because it is already loaded.",
                            &staged_extension_id
                        );
                        continue 'current_extension;
                    }
                } else if updating {
                    // $ NOTIFICATION HERE
                    info!(
                        "Updating loaded extension '{}' from version {} to {}.",
                        &staged_extension_id,
                        loaded_extension_metadata.version,
                        staged_extension_metadata.version
                    );
                    db.unload_extension(&loaded_extension_metadata.id).await?;
                    break 'conflict_check;
                } else if downgrading {
                    // $ PROMPT HERE
                    warn!(
                        "Downgrading loaded extension '{}' from version {} to {}.",
                        &staged_extension_id,
                        loaded_extension_metadata.version,
                        staged_extension_metadata.version
                    );
                    db.unload_extension(&loaded_extension_metadata.id).await?;
                    break 'conflict_check;
                }
            }

            // * Any staged extension can only logically have up to one conflict with a loaded
            // * extension, because of the following reasons:
            // * - Conflicts can only arise when a staged and a loaded extension share the same ID.
            // * - No two loaded extensions can have the same ID due to database constraints.
            // * - No two staged extensions can have the same ID because they are pre-filtered.
            db.load_extension(staged_extension).await?;
            info!("Extension {} loaded.", &staged_extension_id);
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
            metadata: InventoryExtensionMetadata {
                id: InventoryExtensionID::new(&format!("test_{num}")),
                common_name: format!("Test Extension {num}"),
                version: Version::new(1, 0, 0),
            },
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
            metadata: InventoryExtensionMetadata {
                id: InventoryExtensionID::new(&toml.extension_id),
                common_name: toml.extension_common_name,
                version: Version::from_str(&toml.extension_version).unwrap(),
            },
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
    use tracing_subscriber::layer::SubscriberExt;
    use tracing_subscriber::util::SubscriberInitExt;
    use tracing_subscriber::{fmt, EnvFilter};

    use super::{ExtensionManager, InventoryExtension};
    use crate::database::Database;
    use crate::models::common::{Classification, Device, InventoryExtensionID, Manufacturer};

    fn init_logger() {
        let test_writer = fmt::layer().with_test_writer();
        let builder = fmt::Subscriber::builder().with_env_filter(EnvFilter::from_default_env());
        let subscriber = builder.finish();
        let _ = subscriber.with(test_writer).try_init();
    }

    #[tokio::test]
    #[ignore = "not implemented"]
    /// Tests that two extensions with the same ID, but incompatible metadata, will cause an error.
    async fn incompatible_duplicate_extensions() {
        init_logger();
        let db = Database::connect_with_name("test_incompatible_duplicate_extensions").await;
        db.teardown().await;
        todo!()
    }

    #[tokio::test]
    /// Tests that two extensions with the same ID and metadata will not be reloaded or cause a
    /// conflict, even if they have different contents.
    async fn compatible_duplicate_extensions() {
        init_logger();
        let db = Database::connect_with_name("test_compatible_duplicate_extensions").await;

        // Create two extensions with the same ID and metadata, but different contents
        let mut original_extension = InventoryExtension::test(1);
        let mut duplicate_extension = original_extension.clone();
        // Add a different manufacturer to each extension
        let manufacturer_1 = Manufacturer::test(1, &original_extension.metadata.id);
        let manufacturer_2 = Manufacturer::test(2, &duplicate_extension.metadata.id);
        original_extension
            .manufacturers
            .push(manufacturer_1.clone());
        duplicate_extension
            .manufacturers
            .push(manufacturer_2.clone());
        // Add a different classification to each extension
        let classification_1 = Classification::test(1, &original_extension.metadata.id);
        let classification_2 = Classification::test(2, &duplicate_extension.metadata.id);
        original_extension
            .classifications
            .push(classification_1.clone());
        duplicate_extension
            .classifications
            .push(classification_2.clone());
        // Add a different device to each extension
        let device_1 = Device::test(
            1,
            &original_extension.metadata.id,
            &manufacturer_1.id,
            &classification_1.id,
        );
        let device_2 = Device::test(
            2,
            &duplicate_extension.metadata.id,
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
        init_logger();
        let db = Database::connect_with_name("test_reload_extension_update").await;

        // Create two extensions with the same ID, but different versions
        let mut original_extension = InventoryExtension::test(1);
        let mut updated_extension = InventoryExtension::test(1);
        updated_extension.metadata.version = Version::new(1, 0, 1);
        // Add a different manufacturer to each extension
        let manufacturer_1 = Manufacturer::test(1, &original_extension.metadata.id);
        let manufacturer_2 = Manufacturer::test(2, &updated_extension.metadata.id);
        original_extension
            .manufacturers
            .push(manufacturer_1.clone());
        updated_extension.manufacturers.push(manufacturer_2.clone());
        // Add a different classification to each extension
        let classification_1 = Classification::test(1, &original_extension.metadata.id);
        let classification_2 = Classification::test(2, &updated_extension.metadata.id);
        original_extension
            .classifications
            .push(classification_1.clone());
        updated_extension
            .classifications
            .push(classification_2.clone());
        // Add a different device to each extension
        let device_1 = Device::test(
            1,
            &original_extension.metadata.id,
            &manufacturer_1.id,
            &classification_1.id,
        );
        let device_2 = Device::test(
            2,
            &updated_extension.metadata.id,
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
        init_logger();
        let db = Database::connect_with_name("test_reload_extension_override").await;

        // Create two extensions with the same metadata, but with a load override
        let mut original_extension = InventoryExtension::test(1);
        let mut reloaded_extension = InventoryExtension::test(1);
        reloaded_extension.load_override = true;
        // Add a different manufacturer to each extension
        let manufacturer_1 = Manufacturer::test(1, &original_extension.metadata.id);
        let manufacturer_2 = Manufacturer::test(2, &reloaded_extension.metadata.id);
        original_extension
            .manufacturers
            .push(manufacturer_1.clone());
        reloaded_extension
            .manufacturers
            .push(manufacturer_2.clone());
        // Add a different classification to each extension
        let classification_1 = Classification::test(1, &original_extension.metadata.id);
        let classification_2 = Classification::test(2, &reloaded_extension.metadata.id);
        original_extension
            .classifications
            .push(classification_1.clone());
        reloaded_extension
            .classifications
            .push(classification_2.clone());
        // Add a different device to each extension
        let device_1 = Device::test(
            1,
            &original_extension.metadata.id,
            &manufacturer_1.id,
            &classification_1.id,
        );
        let device_2 = Device::test(
            2,
            &reloaded_extension.metadata.id,
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
        init_logger();
        let db = Database::connect_with_name("test_unload_builtin_extension").await;
        db.setup_tables().await.unwrap();
        db.setup_reserved_items().await.unwrap();

        // TODO: Match on error variant once custom errors are added
        assert!(db
            .unload_extension(&InventoryExtensionID::new("builtin"))
            .await
            .is_err());

        db.teardown().await;
    }
}
