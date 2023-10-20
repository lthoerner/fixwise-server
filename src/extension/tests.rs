use semver::Version;

use super::{ExtensionID, ExtensionManager as Manager, InventoryExtension as Extension, Metadata};
use crate::database::Database;
use crate::models::common::{Classification, Device, Manufacturer};

#[tokio::test]
#[ignore = "not implemented"]
/// Tests that two extensions with the same ID, but incompatible metadata, will cause an error.
async fn incompatible_duplicate_extensions() {
    let db = Database::connect_with_name("incompatible_duplicate_extensions").await;
    db.teardown().await;
    todo!()
}

#[tokio::test]
/// Tests that two extensions with the same ID and metadata will not be reloaded or cause a
/// conflict, even if they have different contents.
async fn compatible_duplicate_extensions() {
    let db = Database::connect_with_name("compatible_duplicate_extensions").await;
    let load_override = false;

    // Create two extensions with the same ID and metadata, but different contents
    let mut original_extension = Extension::test(1);
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
    let manager = Manager::with_extensions([original_extension.clone()]);
    manager.load_extensions(&db, load_override).await.unwrap();
    // Make sure the extension was loaded correctly
    db.only_contains(&original_extension).await;
    // Load the second extension into the database
    let manager = Manager::with_extensions([duplicate_extension.clone()]);
    manager.load_extensions(&db, load_override).await.unwrap();
    // Make sure the second extension was not loaded
    db.only_contains(&original_extension).await;

    db.teardown().await;
}

#[tokio::test]
/// Tests that an extension will be replaced by an updated version of itself.
async fn reload_extension_update() {
    let db = Database::connect_with_name("reload_extension_update").await;
    let load_override = false;

    // Create two extensions with the same ID, but different versions
    let mut original_extension = Extension::test(1);
    let mut updated_extension = original_extension.clone();
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
    let manager = Manager::with_extensions([original_extension.clone()]);
    manager.load_extensions(&db, load_override).await.unwrap();
    // Make sure the extension was loaded correctly
    db.only_contains(&original_extension).await;
    // Reload the extension with the updated version, which should unload the original extension
    let manager = Manager::with_extensions([updated_extension.clone()]);
    manager.load_extensions(&db, load_override).await.unwrap();
    // Make sure the original extension was unloaded and the new version was loaded
    db.only_contains(&updated_extension).await;

    db.teardown().await;
}

#[tokio::test]
/// Tests that an extension will be replaced by the same extension with the load override flag.
async fn reload_extension_override() {
    let db = Database::connect_with_name("reload_extension_override").await;
    let load_override = true;

    // Create two extensions with the same metadata, but with developer mode enabled
    let mut original_extension = Extension::test(1);
    let mut reloaded_extension = original_extension.clone();
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
    let manager = Manager::with_extensions([original_extension.clone()]);
    manager.load_extensions(&db, load_override).await.unwrap();
    // Make sure the extension was loaded correctly
    db.only_contains(&original_extension).await;
    // Reload the extension, which should unload the original extension
    let manager = Manager::with_extensions([reloaded_extension.clone()]);
    manager.load_extensions(&db, load_override).await.unwrap();
    // Make sure the original extension was unloaded and the new version was loaded
    db.only_contains(&reloaded_extension).await;

    db.teardown().await;
}

#[tokio::test]
async fn unload_builtin_extension() {
    let db = Database::connect_with_name("unload_builtin_extension").await;
    db.setup_tables().await.unwrap();
    db.setup_reserved_items().await.unwrap();

    // TODO: Match on error variant once custom errors are added
    assert!(db
        .unload_extension(&ExtensionID::new("builtin"))
        .await
        .is_err());

    db.teardown().await;
}

impl Extension {
    /// Creates a basic extension for testing purposes.
    /// Can be modified to test different scenarios.
    #[cfg(test)]
    #[allow(dead_code)]
    pub fn test(num: u32) -> Self {
        Self {
            metadata: Metadata {
                id: ExtensionID::new(&format!("test_{num}")),
                common_name: format!("Test Extension {num}"),
                version: Version::new(1, 0, 0),
            },
            manufacturers: Vec::new(),
            classifications: Vec::new(),
            devices: Vec::new(),
        }
    }
}

impl Manager {
    /// Creates a manager for the provided extensions.
    #[cfg(test)]
    #[allow(dead_code)]
    pub fn with_extensions(extensions: impl IntoIterator<Item = Extension>) -> Self {
        let mut manager = Self::default();
        for extension in extensions {
            manager.staged_extensions.push(extension);
        }

        manager
    }
}
