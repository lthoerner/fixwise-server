mod database;
mod extension;
mod models;

use database::Database;
use extension::ExtensionManager;
use tracing::info;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();

    info!("TechTriage v{}", env!("CARGO_PKG_VERSION"));
    info!("Starting server...");

    info!("Connecting and authenticating to database...");
    let db = Database::connect().await;
    info!("Database connection established.");

    db.setup_tables().await?;
    db.setup_reserved_items().await?;

    info!("Loading inventory extensions...");
    let manager = ExtensionManager::new()?;
    manager.load_extensions(&db).await?;
    info!("All inventory extensions loaded.");

    stop(0);
}

/// Exits the program with a friendly log message instead of an ugly panic message.
fn stop(code: i32) -> ! {
    info!("Stopping server...");
    std::process::exit(code);
}
