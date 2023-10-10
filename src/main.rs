mod customer;
mod database;
mod extension;
mod inventory;
mod ticket;

use database::Database;
use extension::ExtensionManager;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let db = Database::connect().await;
    db.setup_tables().await?;
    db.setup_reserved_items().await?;

    let manager = ExtensionManager::new()?;
    manager.load_extensions(&db).await?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use semver::Version;

    use crate::database::models::{
        Classification, Device, InventoryExtensionID, InventoryExtensionInfo, Manufacturer,
    };
    use crate::database::Database;
    use crate::extension::{ExtensionManager, InventoryExtension};

    #[tokio::test]
    async fn incompatible_duplicate_extensions() {
        let db = Database::connect_with_name("test_incompatible_duplicate_extensions").await;

        db.teardown().await;

        todo!()
    }

    #[tokio::test]
    async fn compatible_duplicate_extensions() {
        let db = Database::connect_with_name("test_compatible_duplicate_extensions").await;

        db.teardown().await;

        todo!()
    }

    #[tokio::test]
    async fn reload_extension_update() {
        let db = Database::connect_with_name("test_reload_extension_update").await;

        // Create two extensions with the same ID, but different versions
        let mut extension_original = InventoryExtension::test(1);
        let mut extension_updated = InventoryExtension::test(1);
        extension_updated.version = Version::new(1, 0, 1);
        // Add a different manufacturer to each extension
        let manufacturer_1 = Manufacturer::test(1, &extension_original.id);
        let manufacturer_2 = Manufacturer::test(2, &extension_updated.id);
        extension_original
            .manufacturers
            .push(manufacturer_1.clone());
        extension_updated.manufacturers.push(manufacturer_2.clone());
        // Add a different classification to each extension
        let classification_1 = Classification::test(1, &extension_original.id);
        let classification_2 = Classification::test(2, &extension_updated.id);
        extension_original
            .classifications
            .push(classification_1.clone());
        extension_updated
            .classifications
            .push(classification_2.clone());
        // Add a different device to each extension
        let device_1 = Device::test(
            1,
            &extension_original.id,
            &manufacturer_1.id,
            &classification_1.id,
        );
        let device_2 = Device::test(
            2,
            &extension_updated.id,
            &manufacturer_2.id,
            &classification_2.id,
        );
        extension_original.devices.push(device_1.clone());
        extension_updated.devices.push(device_2.clone());

        // Load the first extension into the database
        let manager = ExtensionManager::with_extensions([extension_original.clone()]);
        manager.load_extensions(&db).await.unwrap();
        // Make sure the extension was loaded correctly
        assert!(db.only_contains(&extension_original).await.unwrap());
        // Reload the extension with the updated version, which should unload the original extension
        let manager = ExtensionManager::with_extensions([extension_updated.clone()]);
        manager.load_extensions(&db).await.unwrap();
        // Make sure the original extension was unloaded and the new version was loaded
        assert!(db.only_contains(&extension_updated).await.unwrap());

        db.teardown().await;
    }

    #[tokio::test]
    async fn reload_extension_override() {
        let db = Database::connect_with_name("test_reload_extension_override").await;

        // Create two extensions with the same metadata, but with a load override
        let mut extension_original = InventoryExtension::test(1);
        let mut extension_reloaded = InventoryExtension::test(1);
        extension_reloaded.load_override = true;
        // Add a different manufacturer to each extension
        let manufacturer_1 = Manufacturer::test(1, &extension_original.id);
        let manufacturer_2 = Manufacturer::test(2, &extension_reloaded.id);
        extension_original
            .manufacturers
            .push(manufacturer_1.clone());
        extension_reloaded
            .manufacturers
            .push(manufacturer_2.clone());
        // Add a different classification to each extension
        let classification_1 = Classification::test(1, &extension_original.id);
        let classification_2 = Classification::test(2, &extension_reloaded.id);
        extension_original
            .classifications
            .push(classification_1.clone());
        extension_reloaded
            .classifications
            .push(classification_2.clone());
        // Add a different device to each extension
        let device_1 = Device::test(
            1,
            &extension_original.id,
            &manufacturer_1.id,
            &classification_1.id,
        );
        let device_2 = Device::test(
            2,
            &extension_reloaded.id,
            &manufacturer_2.id,
            &classification_2.id,
        );
        extension_original.devices.push(device_1.clone());
        extension_reloaded.devices.push(device_2.clone());

        // Load the first extension into the database
        let manager = ExtensionManager::with_extensions([extension_original.clone()]);
        manager.load_extensions(&db).await.unwrap();
        // Make sure the extension was loaded correctly
        assert!(db.only_contains(&extension_original).await.unwrap());
        // Reload the extension, which should unload the original extension
        let manager = ExtensionManager::with_extensions([extension_reloaded.clone()]);
        manager.load_extensions(&db).await.unwrap();
        // Make sure the original extension was unloaded and the new version was loaded
        assert!(db.only_contains(&extension_reloaded).await.unwrap());

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
