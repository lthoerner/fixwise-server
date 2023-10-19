mod database;
mod extension;
mod models;

use std::sync::OnceLock;

use log::info;
use simplelog::{ColorChoice, Config, LevelFilter, TermLogger, TerminalMode};

use database::Database;
use extension::ExtensionManager;

static DEVELOPER_MODE: OnceLock<bool> = OnceLock::new();

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    use clap::{Arg, ArgAction, Command};
    // * More arguments will most likely be added in later versions
    let args = Command::new("techtriage")
        .bin_name("techtriage")
        .arg(
            Arg::new("developer mode")
                .short('d')
                .long("dev")
                .action(ArgAction::SetTrue)
                .help("Enable developer mode"),
        )
        .get_matches();

    DEVELOPER_MODE.get_or_init(|| *args.get_one::<bool>("developer mode").unwrap());

    TermLogger::init(
        if *DEVELOPER_MODE.get().unwrap() {
            LevelFilter::Debug
        } else {
            LevelFilter::Info
        },
        Config::default(),
        TerminalMode::Stderr,
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

    stop(0);
}

/// Exits the program with a friendly log message instead of an ugly panic message.
fn stop(code: i32) -> ! {
    info!("Stopping server...");
    std::process::exit(code);
}
