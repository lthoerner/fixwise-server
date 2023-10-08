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
