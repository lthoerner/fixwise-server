pub mod models;

use std::future::IntoFuture;

use futures_util::future;
use surrealdb::engine::remote::ws::{Client, Ws};
use surrealdb::opt::auth::Root;
use surrealdb::Surreal;

use self::models::{
    Classification, ClassificationID, ClassificationPullRecord, ClassificationPushRecord,
    DevicePushRecord, GenericPullRecord, InventoryExtensionInfo, InventoryExtensionInfoPullRecord,
    InventoryExtensionInfoPushRecord, Manufacturer, ManufacturerID, ManufacturerPullRecord,
    ManufacturerPushRecord,
};
use crate::extension::InventoryExtension;

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
                DEFINE FIELD extensions ON TABLE {1} TYPE array<record({0})>;
                DEFINE FIELD extensions.* ON TABLE {1} TYPE record({0});

                DEFINE TABLE {2} SCHEMAFUL;
                DEFINE FIELD common_name ON TABLE {2} TYPE string;
                DEFINE FIELD extensions ON TABLE {2} TYPE array<record({0})>;
                DEFINE FIELD extensions.* ON TABLE {2} TYPE record({0});

                DEFINE TABLE {3} SCHEMAFUL;
                DEFINE FIELD common_name ON TABLE {3} TYPE string;
                DEFINE FIELD manufacturer ON TABLE {3} TYPE record({1});
                DEFINE FIELD classification ON TABLE {3} TYPE record({2});
                DEFINE FIELD extension ON TABLE {3} TYPE record({0});
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
                INSERT INTO {0} {{id: \"builtin\", common_name: \"Built-in\"}};
                INSERT INTO {1} [
                    {{
                        id: \"apple\",
                        common_name: \"Apple\",
                        extensions: [\"{0}:builtin\"]
                    }},
                    {{
                        id: \"samsung\",
                        common_name: \"Samsung\",
                        extensions: [\"{0}:builtin\"]
                    }},
                    {{
                        id: \"google\",
                        common_name: \"Google\",
                        extensions: [\"{0}:builtin\"]
                    }},
                    {{
                        id: \"motorola\",
                        common_name: \"Motorola\",
                        extensions: [\"{0}:builtin\"]
                    }},
                    {{
                        id: \"lg\",
                        common_name: \"LG\",
                        extensions: [\"{0}:builtin\"]
                    }},
                ];
                INSERT INTO {2} [
                    {{
                        id: \"phone\",
                        common_name: \"Phone\",
                        extensions: [\"{0}:builtin\"]
                    }},
                    {{
                        id: \"tablet\",
                        common_name: \"Tablet\",
                        extensions: [\"{0}:builtin\"]
                    }},
                    {{
                        id: \"console\",
                        common_name: \"Console\",
                        extensions: [\"{0}:builtin\"]
                    }},
                    {{
                        id: \"laptop\",
                        common_name: \"Laptop\",
                        extensions: [\"{0}:builtin\"]
                    }},
                    {{
                        id: \"desktop\",
                        common_name: \"Desktop\",
                        extensions: [\"{0}:builtin\"]
                    }},
                ];
                ",
                EXTENSION_TABLE_NAME, MANUFACTURER_TABLE_NAME, CLASSIFICATION_TABLE_NAME
            ))
            .await?;

        Ok(())
    }

    /// Loads the contents of an inventory extension into the database.
    pub async fn load_extension(&self, extension: InventoryExtension) -> anyhow::Result<()> {
        self.connection
            .create::<Vec<GenericPullRecord>>(EXTENSION_TABLE_NAME)
            .content(InventoryExtensionInfoPushRecord::from(
                &InventoryExtensionInfo::from(&extension),
            ))
            .await?;

        let mut futures = Vec::new();
        for classification in extension.classifications {
            futures.push(self.add_classification(classification));
        }
        future::join_all(futures).await;

        let mut futures = Vec::new();
        for manufacturer in extension.manufacturers {
            futures.push(self.add_manufacturer(manufacturer));
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

    pub async fn unload_extension(
        &self,
        extension_info: &InventoryExtensionInfo,
    ) -> anyhow::Result<()> {
        self.connection
            .query(&format!(
                "
                DELETE {MANUFACTURER_TABLE_NAME} WHERE extensions = [\"{0}\"];
                DELETE {CLASSIFICATION_TABLE_NAME} WHERE extensions = [\"{0}\"];
                DELETE {DEVICE_TABLE_NAME} WHERE extension = \"{0}\";
                DELETE {EXTENSION_TABLE_NAME} WHERE id = \"{0}\";
                ",
                extension_info.id.to_namespaced_string()
            ))
            .await?;

        Ok(())
    }

    /// Lists all currently-loaded extensions in the database.
    pub async fn list_extensions(&self) -> anyhow::Result<Vec<InventoryExtensionInfo>> {
        let pull_records = self
            .connection
            .select::<Vec<InventoryExtensionInfoPullRecord>>(EXTENSION_TABLE_NAME)
            .await?;

        let mut extensions = Vec::new();
        for record in pull_records {
            extensions.push(InventoryExtensionInfo::try_from(record)?);
        }

        Ok(extensions)
    }

    async fn add_manufacturer(&self, mut manufacturer: Manufacturer) -> anyhow::Result<()> {
        if let Some(existing_record) = self.get_manufacturer(&manufacturer.id).await? {
            manufacturer.merge(existing_record.try_into()?);
        }

        self.connection
            .create::<Vec<GenericPullRecord>>(MANUFACTURER_TABLE_NAME)
            .content(ManufacturerPushRecord::from(&manufacturer))
            .await?;

        Ok(())
    }

    async fn add_classification(&self, mut classification: Classification) -> anyhow::Result<()> {
        if let Some(existing_record) = self.get_classification(&classification.id).await? {
            classification.merge(existing_record.try_into()?);
        }

        self.connection
            .create::<Vec<GenericPullRecord>>(CLASSIFICATION_TABLE_NAME)
            .content(ClassificationPushRecord::from(&classification))
            .await?;

        Ok(())
    }

    async fn get_manufacturer(
        &self,
        id: &ManufacturerID,
    ) -> anyhow::Result<Option<ManufacturerPullRecord>> {
        Ok(self
            .connection
            .select::<Option<ManufacturerPullRecord>>((
                MANUFACTURER_TABLE_NAME,
                id.to_non_namespaced_string(),
            ))
            .await?)
    }

    async fn get_classification(
        &self,
        id: &ClassificationID,
    ) -> anyhow::Result<Option<ClassificationPullRecord>> {
        Ok(self
            .connection
            .select::<Option<ClassificationPullRecord>>((
                CLASSIFICATION_TABLE_NAME,
                id.to_non_namespaced_string(),
            ))
            .await?)
    }
}
