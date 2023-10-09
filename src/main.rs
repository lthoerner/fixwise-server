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

    // ? How should duplicate device classifications and manufacturers be handled?

    Ok(())
}

#[cfg(test)]
mod tests {
    use semver::Version;

    use crate::database::models::{InventoryExtensionID, InventoryExtensionInfo};
    use crate::database::Database;

    #[tokio::test]
    async fn incompatible_duplicate_extensions() {
        let db = Database::connect_with_name("test_incompatible_duplicate_extensions").await;
        db.setup_tables().await.unwrap();
        db.setup_reserved_items().await.unwrap();

        db.teardown().await;

        todo!()
    }

    #[tokio::test]
    async fn compatible_duplicate_extensions() {
        let db = Database::connect_with_name("test_compatible_duplicate_extensions").await;
        db.setup_tables().await.unwrap();
        db.setup_reserved_items().await.unwrap();

        db.teardown().await;

        todo!()
    }

    #[tokio::test]
    async fn reload_extension_no_override() {
        let db = Database::connect_with_name("test_reload_extension_no_override").await;

        db.teardown().await;

        todo!()
    }

    #[tokio::test]
    async fn reload_extension_with_override() {
        let db = Database::connect_with_name("test_reload_extension_with_override").await;

        db.teardown().await;

        todo!()
    }

    #[tokio::test]
    async fn unload_builtin_extension() {
        let db = Database::connect_with_name("test_unload_builtin_extension").await;
        db.setup_tables().await.unwrap();
        db.setup_reserved_items().await.unwrap();

        // TODO: Match on error variant onece custom errors are added
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
