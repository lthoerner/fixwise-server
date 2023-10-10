pub mod models;

use std::future::IntoFuture;
use std::net::{Ipv4Addr, SocketAddr};

use futures_util::future;
use surrealdb::engine::remote::ws::{Client, Ws};
use surrealdb::opt::auth::Root;
use surrealdb::Surreal;

use self::models::{
    Classification, ClassificationID, ClassificationPullRecord, ClassificationPushRecord, Device,
    DevicePullRecord, DevicePushRecord, GenericPullRecord, InventoryExtensionID,
    InventoryExtensionInfo, InventoryExtensionInfoPullRecord, InventoryExtensionInfoPushRecord,
    Manufacturer, ManufacturerID, ManufacturerPullRecord, ManufacturerPushRecord,
};
use crate::extension::InventoryExtension;

const EXTENSION_TABLE_NAME: &str = "extensions";
const MANUFACTURER_TABLE_NAME: &str = "manufacturers";
const CLASSIFICATION_TABLE_NAME: &str = "classifications";
const DEVICE_TABLE_NAME: &str = "devices";

/// Wrapper type for a SurrealDB connection.
pub struct Database {
    connection: Surreal<Client>,
    #[allow(dead_code)]
    config: DatabaseConfig,
}

/// Configuration for connecting to the database.
pub struct DatabaseConfig {
    pub address: SocketAddr,
    pub username: String,
    pub password: String,
    pub namespace: String,
    pub database: String,
}

impl Default for DatabaseConfig {
    fn default() -> Self {
        DatabaseConfig {
            address: (Ipv4Addr::LOCALHOST, 8000).into(),
            username: "root".to_owned(),
            password: "root".to_owned(),
            namespace: "test".to_owned(),
            database: "test".to_owned(),
        }
    }
}

impl Database {
    /// Connects to the database, if it is available, using the default configuration.
    pub async fn connect() -> Self {
        Self::connect_with_config(DatabaseConfig::default()).await
    }

    /// Connects to the database using defaults except for the provided database name.
    #[cfg(test)]
    #[allow(dead_code)]
    pub async fn connect_with_name(database: &str) -> Self {
        Self::connect_with_config(DatabaseConfig {
            database: database.to_owned(),
            ..Default::default()
        })
        .await
    }

    /// Connects to the database using the provided configuration.
    pub async fn connect_with_config(config: DatabaseConfig) -> Self {
        let connection = Surreal::new::<Ws>(config.address).await.unwrap();
        connection
            .use_ns(&config.namespace)
            .use_db(&config.database)
            .await
            .unwrap();
        connection
            .signin(Root {
                username: &config.username,
                password: &config.password,
            })
            .await
            .unwrap();

        Self { connection, config }
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
        // * The double braces are required to escape their meaning in a formatting literal.
        self.connection
            .query(&format!(
                "
                INSERT INTO {0} {{
                    id: \"builtin\",
                    common_name: \"Built-in\",
                    version: \"0.0.0\"
                }};

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

    /// Deletes all items from the database, but leaves the schema intact.
    /// Used for testing purposes.
    #[cfg(test)]
    #[allow(dead_code)]
    pub async fn clear(&self) -> anyhow::Result<()> {
        self.connection
            .delete::<Vec<GenericPullRecord>>(EXTENSION_TABLE_NAME)
            .await?;
        self.connection
            .delete::<Vec<GenericPullRecord>>(MANUFACTURER_TABLE_NAME)
            .await?;
        self.connection
            .delete::<Vec<GenericPullRecord>>(CLASSIFICATION_TABLE_NAME)
            .await?;
        self.connection
            .delete::<Vec<GenericPullRecord>>(DEVICE_TABLE_NAME)
            .await?;

        Ok(())
    }

    /// Deletes the current database and all of its contents.
    /// Used by tests so the database instance can be reused.
    #[cfg(test)]
    pub async fn teardown(self) {
        self.connection
            .query(&format!("REMOVE DATABASE {}", self.config.database))
            .await
            .unwrap();
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
        if extension_info.id == InventoryExtensionID::new("builtin") {
            return Err(anyhow::anyhow!("Cannot unload built-in extension"));
        }

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

    /// Lists all the manufacturers in the database.
    #[allow(dead_code)]
    pub async fn list_manufacturers(&self) -> anyhow::Result<Vec<Manufacturer>> {
        let pull_records = self
            .connection
            .select::<Vec<ManufacturerPullRecord>>(MANUFACTURER_TABLE_NAME)
            .await?;

        let mut manufacturers = Vec::new();
        for record in pull_records {
            manufacturers.push(Manufacturer::try_from(record)?);
        }

        Ok(manufacturers)
    }

    /// Lists all the classifications in the database.
    #[allow(dead_code)]
    pub async fn list_classifications(&self) -> anyhow::Result<Vec<Classification>> {
        let pull_records = self
            .connection
            .select::<Vec<ClassificationPullRecord>>(CLASSIFICATION_TABLE_NAME)
            .await?;

        let mut classifications = Vec::new();
        for record in pull_records {
            classifications.push(Classification::try_from(record)?);
        }

        Ok(classifications)
    }

    /// Lists all the devices in the database.
    pub async fn list_devices(&self) -> anyhow::Result<Vec<Device>> {
        let pull_records = self
            .connection
            .select::<Vec<DevicePullRecord>>(DEVICE_TABLE_NAME)
            .await?;

        let mut devices = Vec::new();
        for record in pull_records {
            devices.push(Device::try_from(record)?);
        }

        Ok(devices)
    }

    /// Adds a manufacturer to the database, merging it with an existing record if needed.
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

    /// Adds a classification to the database, merging it with an existing record if needed.
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

    // ? Can this be combined with `get_classification()` into a single function?
    /// Gets a manufacturer from the database, if it exists.
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

    /// Gets a classification from the database, if it exists.
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

    /// Checks that the database only contains the given extension and its contents.
    /// Used for testing purposes.
    #[cfg(test)]
    #[allow(dead_code)]
    pub async fn only_contains(&self, extension: &InventoryExtension) -> anyhow::Result<bool> {
        let loaded_extension_info = self.list_extensions().await?;
        let loaded_manufacturer_info = self.list_manufacturers().await?;
        let loaded_classification_info = self.list_classifications().await?;
        let loaded_device_info = self.list_devices().await?;

        Ok(loaded_extension_info.len() == 1
            && loaded_extension_info[0] == InventoryExtensionInfo::from(extension)
            && loaded_manufacturer_info == extension.manufacturers
            && loaded_classification_info == extension.classifications
            && loaded_device_info == extension.devices)
    }
}
