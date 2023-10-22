use semver::Version;

use super::conflicts::{LoadConflict, StageConflict, VersionChange};
use super::{ExtensionID, ExtensionManager as Manager, InventoryExtension as Extension, Metadata};
use crate::database::Database;
use crate::models::common::ids::UniqueID;
use crate::models::common::{Device, DeviceClassification, DeviceManufacturer};

#[tokio::test]
/// Tests that an extension which does not already exist in the database will be loaded without
/// causing a conflict.
async fn load_new_extension() {
    let db = Database::connect_with_name("load_new_extension").await;

    // Create a basic extension
    let extension = Extension::test_single(1, 1);
    let load_override = false;

    // Load the extension into the database
    let (manager, stage_conflicts) = Manager::with_extensions([extension.clone()]);
    let load_conflicts = manager.load_extensions(&db, load_override).await.unwrap();
    // Make sure there were no conflicts
    assert_eq!(stage_conflicts.len(), 0);
    assert_eq!(load_conflicts.len(), 0);
    // Make sure the extension was loaded correctly
    db.only_contains(&extension).await;

    db.teardown().await;
}

#[tokio::test]
/// Tests that two extensions with the same ID and metadata will not be reloaded or cause an
/// unresolvable conflict, even if they have different contents.
async fn compatible_duplicate_extensions() {
    let db = Database::connect_with_name("compatible_duplicate_extensions").await;

    // Create two extensions with the same metadata, but different contents
    let (original_extension, duplicate_extension) = Extension::test_pair();
    let load_override = false;

    // Load the extension into the database
    let (manager, stage_conflicts) = Manager::with_extensions([original_extension.clone()]);
    let load_conflicts = manager.load_extensions(&db, load_override).await.unwrap();
    // Make sure there were no conflicts
    assert_eq!(stage_conflicts.len(), 0);
    assert_eq!(load_conflicts.len(), 0);
    // Make sure the extension was loaded correctly
    db.only_contains(&original_extension).await;
    // Load the second extension into the database
    let (manager, stage_conflicts) = Manager::with_extensions([duplicate_extension.clone()]);
    let load_conflicts = manager.load_extensions(&db, load_override).await.unwrap();
    // Make sure the conflicts were correctly identified
    assert_eq!(stage_conflicts.len(), 0);
    assert_eq!(load_conflicts.len(), 1);
    assert_eq!(
        load_conflicts[0],
        LoadConflict::duplicate(duplicate_extension.metadata.id)
    );
    // Make sure the second extension was not loaded
    db.only_contains(&original_extension).await;

    db.teardown().await;
}

#[tokio::test]
/// Tests that an extension will be replaced by an updated version of itself.
async fn reload_extension_update() {
    let db = Database::connect_with_name("reload_extension_update").await;

    // Create two extensions with the same ID, but different versions
    let (original_extension, mut updated_extension) = Extension::test_pair();
    updated_extension.metadata.version = Version::new(1, 0, 1);
    let load_override = false;

    // Load the extension into the database
    let (manager, stage_conflicts) = Manager::with_extensions([original_extension.clone()]);
    let load_conflicts = manager.load_extensions(&db, load_override).await.unwrap();
    // Make sure there were no conflicts
    assert_eq!(stage_conflicts.len(), 0);
    assert_eq!(load_conflicts.len(), 0);
    // Make sure the extension was loaded correctly
    db.only_contains(&original_extension).await;
    // Reload the extension with the updated version, which should unload the original extension
    let (manager, stage_conflicts) = Manager::with_extensions([updated_extension.clone()]);
    let load_conflicts = manager.load_extensions(&db, load_override).await.unwrap();
    // Make sure the conflicts were correctly identified
    assert_eq!(stage_conflicts.len(), 0);
    assert_eq!(load_conflicts.len(), 1);
    assert_eq!(
        load_conflicts[0],
        LoadConflict::version_change(
            original_extension.metadata.id,
            original_extension.metadata.version,
            updated_extension.metadata.version.clone()
        )
    );
    // Make sure the original extension was unloaded and the newer version was loaded
    db.only_contains(&updated_extension).await;

    db.teardown().await;
}

#[tokio::test]
/// Tests that an extension will not be replaced by a downgraded version of itself.
async fn skip_extension_downgrade() {
    let db = Database::connect_with_name("skip_extension_downgrade").await;

    // Create two extensions with the same ID, but different versions
    let (mut original_extension, downgraded_extension) = Extension::test_pair();
    original_extension.metadata.version = Version::new(1, 0, 1);
    let load_override = false;

    // Load the extension into the database
    let (manager, stage_conflicts) = Manager::with_extensions([original_extension.clone()]);
    let load_conflicts = manager.load_extensions(&db, load_override).await.unwrap();
    // Make sure there were no conflicts
    assert_eq!(stage_conflicts.len(), 0);
    assert_eq!(load_conflicts.len(), 0);
    // Make sure the extension was loaded correctly
    db.only_contains(&original_extension).await;
    // Attempt to load the older version of the extension, which should leave the original intact
    let (manager, stage_conflicts) = Manager::with_extensions([downgraded_extension.clone()]);
    let load_conflicts = manager.load_extensions(&db, load_override).await.unwrap();
    // Make sure the conflicts were correctly identified
    assert_eq!(stage_conflicts.len(), 0);
    assert_eq!(load_conflicts.len(), 1);
    assert_eq!(
        load_conflicts[0],
        LoadConflict::version_change(
            downgraded_extension.metadata.id,
            original_extension.metadata.version.clone(),
            downgraded_extension.metadata.version
        )
    );
    // Make sure the original extension was left intact and the older version was not loaded
    db.only_contains(&original_extension).await;

    db.teardown().await;
}

#[tokio::test]
/// Tests that an extension will be replaced by the same extension with the load override flag.
async fn reload_extension_override() {
    let db = Database::connect_with_name("reload_extension_override").await;

    // Create two extensions with the same metadata, but with developer mode enabled
    let (original_extension, reloaded_extension) = Extension::test_pair();
    let load_override = true;

    // Load the extension into the database
    let (manager, stage_conflicts) = Manager::with_extensions([original_extension.clone()]);
    let load_conflicts = manager.load_extensions(&db, load_override).await.unwrap();
    // Make sure there were no conflicts
    assert_eq!(stage_conflicts.len(), 0);
    assert_eq!(load_conflicts.len(), 0);
    // Make sure the extension was loaded correctly
    db.only_contains(&original_extension).await;
    // Reload the extension, which should unload the original extension
    let (manager, stage_conflicts) = Manager::with_extensions([reloaded_extension.clone()]);
    let load_conflicts = manager.load_extensions(&db, load_override).await.unwrap();
    // Make sure the conflicts were correctly identified
    assert_eq!(stage_conflicts.len(), 0);
    assert_eq!(load_conflicts.len(), 1);
    assert_eq!(
        load_conflicts[0],
        LoadConflict::duplicate(original_extension.metadata.id)
    );
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
    /// Creates a basic extension with no contents for testing purposes.
    /// Can be modified to test different scenarios.
    fn test(num: u32) -> Self {
        Self {
            metadata: Metadata {
                id: ExtensionID::new(format!("test_{num}")),
                common_name: format!("Test Extension {num}"),
                version: Version::new(1, 0, 0),
            },
            device_manufacturers: Vec::new(),
            device_classifications: Vec::new(),
            devices: Vec::new(),
        }
    }

    /// Creates a single basic extension with contents.
    /// Can be modified to test different scenarios.
    fn test_single(extension_num: u32, contents_num: u32) -> Self {
        // Create an empty extension.
        let mut extension = Self::test(extension_num);

        // Populate the extension with one device manufacturer, device classification, and device.
        let device_manufacturer = DeviceManufacturer::test(contents_num, &extension.metadata.id);
        let device_classification =
            DeviceClassification::test(contents_num, &extension.metadata.id);
        let device = Device::test(
            contents_num,
            &extension.metadata.id,
            &device_manufacturer.id,
            &device_classification.id,
        );

        extension.device_manufacturers.push(device_manufacturer);
        extension.device_classifications.push(device_classification);
        extension.devices.push(device);

        extension
    }

    /// Creates two basic extensions with the same metadata and different contents.
    /// Can be modified to test different scenarios.
    fn test_pair() -> (Self, Self) {
        (Self::test_single(1, 1), Self::test_single(1, 2))
    }
}

impl Manager {
    /// Creates a manager for the provided extensions.
    fn with_extensions(
        extensions: impl IntoIterator<Item = Extension>,
    ) -> (Self, Vec<StageConflict>) {
        let mut manager = Self::default();
        let mut conflicts = Vec::new();
        for extension in extensions {
            // $ This cannot be an unwrap if it is to be tested
            let conflict = manager.stage_extension(extension).unwrap();
            if let Some(conflict) = conflict {
                conflicts.push(conflict);
            }
        }

        (manager, conflicts)
    }
}

impl LoadConflict {
    /// Creates a duplicate conflict between two copies of the same extension.
    fn duplicate(id: ExtensionID) -> Self {
        Self {
            id,
            version_change: None,
            name_change: None,
        }
    }

    /// Creates a version change conflict between two versions of the same extension.
    fn version_change(id: ExtensionID, loaded_version: Version, staged_version: Version) -> Self {
        Self {
            id,
            version_change: Some(VersionChange {
                loaded_version,
                staged_version,
            }),
            name_change: None,
        }
    }
}
