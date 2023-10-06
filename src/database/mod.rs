pub mod models;

use std::future::IntoFuture;

use futures_util::future;
use surrealdb::engine::remote::ws::{Client, Ws};
use surrealdb::opt::auth::Root;
use surrealdb::Surreal;

use crate::extension::InventoryExtension;
use models::{
    DevicePushRecord, GenericPullRecord, InventoryExtensionInfo, InventoryExtensionInfoPushRecord,
    ManufacturerPushRecord,
};

use self::models::ClassificationPushRecord;

const EXTENSION_TABLE_NAME: &str = "extensions";
const MANUFACTURER_TABLE_NAME: &str = "manufacturers";
const CLASSIFICATION_TABLE_NAME: &str = "classifications";
const DEVICE_TABLE_NAME: &str = "devices";

/// Wrapper type for a SurrealDB connection.
pub struct Database {
    connection: Surreal<Client>,
}

impl Database {
    /// Connects to the database, if it is available.
    pub async fn connect() -> Self {
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

    /// Sets up the tables needed for core functionality.
    pub async fn setup_tables(&self) -> anyhow::Result<()> {
        // * Some notes:
        // * - ID is an implicit field on all tables and uses the `sql::Thing` type.
        // * - Manufacturers and device classifications do not have an `extension` field because
        // *   they can be added by multiple extensions without conflict.
        self.connection
            .query(&format!(
                "
                DEFINE TABLE {0} SCHEMAFUL;
                DEFINE FIELD common_name ON TABLE {0} TYPE string;
                DEFINE FIELD version ON TABLE {0} TYPE string;

                DEFINE TABLE {1} SCHEMAFUL;
                DEFINE FIELD common_name ON TABLE {1} TYPE string;

                DEFINE TABLE {2} SCHEMAFUL;
                DEFINE FIELD common_name ON TABLE {2} TYPE string;

                DEFINE TABLE {3} SCHEMAFUL;
                DEFINE FIELD common_name ON TABLE {3} TYPE string;
                DEFINE FIELD manufacturer ON TABLE {3} TYPE record(manufacturers);
                DEFINE FIELD classification ON TABLE {3} TYPE record(classifications);
                DEFINE FIELD extension ON TABLE {3} TYPE record(extensions);
                DEFINE FIELD primary_model_identifiers ON TABLE {3} TYPE array<string>;
                DEFINE FIELD primary_model_identifiers.* ON TABLE {3} TYPE string;
                DEFINE FIELD extended_model_identifiers ON TABLE {3} TYPE array<string>;
                DEFINE FIELD extended_model_identifiers.* ON TABLE {3} TYPE string;
                ",
                EXTENSION_TABLE_NAME,
                MANUFACTURER_TABLE_NAME,
                CLASSIFICATION_TABLE_NAME,
                DEVICE_TABLE_NAME
            ))
            .await?;

        Ok(())
    }

    /// Sets up IDs for "baked-in" manufacturers and device classifications.
    pub async fn setup_reserved_items(&self) -> anyhow::Result<()> {
        // * The double brackets are required to escape their meaning in a formatting literal.
        self.connection
            .query(&format!(
                "
                INSERT INTO {} [
                    {{id: \"apple\", common_name: \"Apple\"}},
                    {{id: \"samsung\", common_name: \"Samsung\"}},
                    {{id: \"google\", common_name: \"Google\"}},
                    {{id: \"motorola\", common_name: \"Motorola\"}},
                    {{id: \"lg\", common_name: \"LG\"}},
                ];
                INSERT INTO {} [
                    {{id: \"phone\", common_name: \"Phone\"}},
                    {{id: \"tablet\", common_name: \"Tablet\"}},
                    {{id: \"console\", common_name: \"Console\"}},
                    {{id: \"laptop\", common_name: \"Laptop\"}},
                    {{id: \"desktop\", common_name: \"Desktop\"}},
                ];
                ",
                MANUFACTURER_TABLE_NAME, CLASSIFICATION_TABLE_NAME
            ))
            .await?;

        Ok(())
    }

    /// Adds the contents of an inventory extension to the database.
    pub async fn add_extension(&self, extension: InventoryExtension) -> anyhow::Result<()> {
        self.connection
            .create::<Vec<GenericPullRecord>>(EXTENSION_TABLE_NAME)
            .content(InventoryExtensionInfoPushRecord::from(
                &InventoryExtensionInfo::from(&extension),
            ))
            .await?;

        let mut futures = Vec::new();
        for classification in extension.classifications {
            futures.push(
                self.connection
                    .create::<Vec<GenericPullRecord>>(CLASSIFICATION_TABLE_NAME)
                    .content(ClassificationPushRecord::from(&classification))
                    .into_future(),
            );
        }
        future::join_all(futures).await;

        let mut futures = Vec::new();
        for manufacturer in extension.manufacturers {
            futures.push(
                self.connection
                    .create::<Vec<GenericPullRecord>>(MANUFACTURER_TABLE_NAME)
                    .content(ManufacturerPushRecord::from(&manufacturer))
                    .into_future(),
            );
        }
        future::join_all(futures).await;

        let mut futures = Vec::new();
        for device in extension.devices {
            futures.push(
                self.connection
                    .create::<Vec<GenericPullRecord>>(DEVICE_TABLE_NAME)
                    .content(DevicePushRecord::from(&device))
                    .into_future(),
            )
        }
        future::try_join_all(futures).await?;

        Ok(())
    }

    pub async fn list_extensions(&self) {
        todo!()
    }
}
