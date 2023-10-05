mod customer;
mod inventory;
mod ticket;

use std::future::IntoFuture;
use std::str::FromStr;

use futures_util::future;
use serde::{Deserialize, Serialize};
use surrealdb::engine::remote::ws::{Client, Ws};
use surrealdb::opt::auth::Root;
use surrealdb::sql::Thing;
use surrealdb::Surreal;

#[derive(Debug, Deserialize)]
struct GenericRecord {
    // * This has to be an attribute tag because `_id` does not map to `id`.
    #[allow(dead_code)]
    id: Thing,
}

#[derive(Debug, Serialize, Deserialize)]
struct Manufacturer {
    id: String,
    common_name: String,
}

#[derive(Debug, Serialize)]
struct Device {
    id: String,
    manufacturer: Thing,
    kind: Thing,
    common_name: String,
    primary_model_identifiers: Vec<String>,
    extended_model_identifiers: Vec<String>,
}

/// Represents an extension which can be added to the database.
#[derive(Debug)]
struct Extension {
    // name: String,
    manufacturers: Vec<Manufacturer>,
    device_kinds: Vec<String>,
    devices: Vec<Device>,
}

struct Database {
    connection: Surreal<Client>,
}

impl Database {
    async fn connect() -> Self {
        let connection = Surreal::new::<Ws>("localhost:8000").await.unwrap();
        connection.use_ns("test").use_db("test").await.unwrap();
        connection
            .signin(Root {
                username: "root",
                password: "root",
            })
            .await
            .unwrap();

        Self { connection }
    }

    async fn setup_tables(&self) -> anyhow::Result<()> {
        // ? Do device kinds need a common name?
        self.connection
            .query(
                "
                DEFINE TABLE manufacturers SCHEMAFUL;
                DEFINE FIELD common_name ON TABLE manufacturers TYPE string;
                DEFINE TABLE device_kinds SCHEMAFUL;
                DEFINE TABLE devices SCHEMAFUL;
                DEFINE FIELD common_name ON TABLE devices TYPE string;
                DEFINE FIELD manufacturer ON TABLE devices TYPE record(manufacturers);
                DEFINE FIELD kind ON TABLE devices TYPE record(device_kinds);
                DEFINE FIELD primary_model_identifiers ON TABLE devices TYPE array<string>;
                DEFINE FIELD primary_model_identifiers.* ON TABLE devices TYPE string;
                DEFINE FIELD extended_model_identifiers ON TABLE devices TYPE array<string>;
                DEFINE FIELD extended_model_identifiers.* ON TABLE devices TYPE string;
                ",
            )
            .await?;

        Ok(())
    }

    async fn setup_reserved_items(&self) -> anyhow::Result<()> {
        self.connection
            .query(
                "
                INSERT INTO manufacturers [
                    {id: \"apple\", common_name: \"Apple\"},
                    {id: \"samsung\", common_name: \"Samsung\"},
                    {id: \"google\", common_name: \"Google\"},
                    {id: \"motorola\", common_name: \"Motorola\"},
                    {id: \"lg\", common_name: \"LG\"},
                ];
                INSERT INTO device_kinds [
                    {id: \"phone\"},
                    {id: \"tablet\"},
                    {id: \"console\"},
                    {id: \"laptop\"},
                    {id: \"desktop\"},
                ];
                ",
            )
            .await?;

        Ok(())
    }

    async fn add_extension(&self, ext: Extension) -> anyhow::Result<()> {
        let mut futures = Vec::new();
        for kind in ext.device_kinds {
            futures.push(
                self.connection
                    .create::<Option<GenericRecord>>(("device_kinds", &kind))
                    .into_future(),
            );
        }
        future::join_all(futures).await;

        let mut futures = Vec::new();
        for manufacturer in ext.manufacturers {
            futures.push(
                self.connection
                    .create::<Vec<GenericRecord>>("manufacturers")
                    .content(manufacturer)
                    .into_future(),
            );
        }
        future::join_all(futures).await;

        let mut futures = Vec::new();
        for device in ext.devices {
            futures.push(
                self.connection
                    .create::<Vec<GenericRecord>>("devices")
                    .content(device)
                    .into_future(),
            )
        }
        future::try_join_all(futures).await?;

        Ok(())
    }
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let db = Database::connect().await;
    db.setup_tables().await?;
    db.setup_reserved_items().await?;

    let ext = load_extension("extensions/iphone_all.toml")?;
    db.add_extension(ext).await?;

    // ? How should duplicate device kinds and manufacturers be handled?

    Ok(())
}

/// Represents an extension read from a TOML file.
/// Some types are not compatible with the database, so this type must be converted into an
/// `Extension` before calling `Database::add_extension()`.
#[derive(Debug, Deserialize)]
struct ExtensionToml {
    extension_name: String,
    manufacturers: Vec<Manufacturer>,
    device_kinds: Option<Vec<String>>,
    devices: Vec<DeviceToml>,
}

impl From<ExtensionToml> for Extension {
    fn from(ext: ExtensionToml) -> Self {
        let name = ext.extension_name;
        let devices = ext
            .devices
            .into_iter()
            // ? Is there a more conventional way to do this conversion?
            .map(|d| Device {
                id: d.generate_id(&name),
                manufacturer: Thing::from_str(&["manufacturers", &d.manufacturer].join(":"))
                    .unwrap(),
                kind: Thing::from_str(&["device_kinds", &d.kind].join(":")).unwrap(),
                common_name: d.common_name,
                primary_model_identifiers: d.primary_model_identifiers,
                extended_model_identifiers: d.extended_model_identifiers,
            })
            .collect();

        Extension {
            manufacturers: ext.manufacturers,
            device_kinds: ext.device_kinds.unwrap_or_default(),
            devices,
        }
    }
}

#[derive(Debug, Deserialize)]
struct DeviceToml {
    // TODO: Figure out a better name for this
    true_name: String,
    manufacturer: String,
    kind: String,
    common_name: String,
    primary_model_identifiers: Vec<String>,
    extended_model_identifiers: Vec<String>,
}

impl DeviceToml {
    /// Generates a namespaced ID for the device, allowing devices of different extensions,
    /// manufacturers, or kinds to share the same name.
    fn generate_id(&self, extension_name: &str) -> String {
        [
            extension_name,
            &self.manufacturer,
            &self.kind,
            &self.true_name,
        ]
        .join("/")
    }
}

/// Parses a TOML file into an extension, which can then be added to the database.
fn load_extension(filename: &str) -> anyhow::Result<Extension> {
    // ? Is it any better to read to bytes and convert to struct or is string fine?
    let toml = std::fs::read_to_string(filename)?;
    let extension: ExtensionToml = toml::from_str(&toml)?;
    Ok(extension.into())
}
