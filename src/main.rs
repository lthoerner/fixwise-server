mod database;
mod extension;
mod models;

use log::info;
use simplelog::{ColorChoice, Config, LevelFilter, TermLogger, TerminalMode};

use database::Database;
use extension::ExtensionManager;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    TermLogger::init(
        LevelFilter::Debug,
        Config::default(),
        // * Stdout is used because stderr is conventionally used for any non-standard output.
        // * In the case of this server software, the logs are the standard output.
        TerminalMode::Stdout,
        ColorChoice::Auto,
    )?;

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

    Ok(())
}

/// Exits the program with a friendly log message instead of an ugly panic message.
fn stop(code: i32) -> ! {
    info!("Stopping server...");
    std::process::exit(code);
}
