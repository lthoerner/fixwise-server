use std::future::IntoFuture;
use std::net::{Ipv4Addr, SocketAddr};

use futures_util::future;
use log::{debug, error, info, warn};
use surrealdb::engine::remote::ws::{Client, Ws};
use surrealdb::opt::auth::Root;
use surrealdb::Surreal;

use crate::extensions::InventoryExtension;
use crate::models::common::{
    Device, DeviceClassification, DeviceClassificationUniqueID, DeviceManufacturer,
    DeviceManufacturerUniqueID, InventoryExtensionMetadata, InventoryExtensionUniqueID, UniqueID,
};
use crate::models::database::{
    DeviceClassificationPullRecord, DeviceClassificationPushRecord, DeviceManufacturerPullRecord,
    DeviceManufacturerPushRecord, DevicePullRecord, DevicePushRecord, GenericPullRecord,
    InventoryExtensionMetadataPullRecord, InventoryExtensionMetadataPushRecord,
};
use crate::stop;

// TODO: Find a more sensible place to move these
pub const EXTENSION_TABLE_NAME: &str = "extensions";
pub const DEVICE_MANUFACTURER_TABLE_NAME: &str = "device_manufacturers";
pub const DEVICE_CLASSIFICATION_TABLE_NAME: &str = "device_classifications";
pub const DEVICE_TABLE_NAME: &str = "devices";

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
        debug!(
            "Connecting to database [NS: '{}', DB: '{}'] at {}",
            config.namespace, config.database, config.address
        );

        let Ok(connection) = Surreal::new::<Ws>(config.address).await else {
            error!("Failed to connect to database. Please make sure it is running.");
            stop(1);
        };

        connection
            .use_ns(&config.namespace)
            .use_db(&config.database)
            .await
            .unwrap_or_else(|_| {
                error!("Failed to select namespace and database from SurrealDB instance.");
                stop(2);
            });

        connection
            .signin(Root {
                username: &config.username,
                password: &config.password,
            })
            .await
            .unwrap_or_else(|_| {
                error!("Failed to sign into SurrealDB instance. Please check your credentials.");
                stop(3);
            });

        Self { connection, config }
    }

    /// Sets up the tables and schema needed for core functionality.
    /// If the tables already exist, this will do nothing.
    pub async fn setup_tables(&self) -> anyhow::Result<()> {
        info!("Setting up database tables/schema...");

        // * ID is an implicit field on all tables and uses the [`sql::Thing`] type.
        self.connection
            .query(&format!(
                "
                DEFINE TABLE {EXTENSION_TABLE_NAME} SCHEMAFUL;
                DEFINE FIELD common_name ON TABLE {EXTENSION_TABLE_NAME} TYPE string;
                DEFINE FIELD version ON TABLE {EXTENSION_TABLE_NAME} TYPE string;

                DEFINE TABLE {DEVICE_MANUFACTURER_TABLE_NAME} SCHEMAFUL;
                DEFINE FIELD common_name ON TABLE {DEVICE_MANUFACTURER_TABLE_NAME} TYPE string;
                DEFINE FIELD extensions ON TABLE {DEVICE_MANUFACTURER_TABLE_NAME} TYPE array<record({EXTENSION_TABLE_NAME})>;

                DEFINE TABLE {DEVICE_CLASSIFICATION_TABLE_NAME} SCHEMAFUL;
                DEFINE FIELD common_name ON TABLE {DEVICE_CLASSIFICATION_TABLE_NAME} TYPE string;
                DEFINE FIELD extensions ON TABLE {DEVICE_CLASSIFICATION_TABLE_NAME} TYPE array<record({EXTENSION_TABLE_NAME})>;

                DEFINE TABLE {DEVICE_TABLE_NAME} SCHEMAFUL;
                DEFINE FIELD internal_id ON TABLE {DEVICE_TABLE_NAME} TYPE string;
                DEFINE FIELD common_name ON TABLE {DEVICE_TABLE_NAME} TYPE string;
                DEFINE FIELD manufacturer ON TABLE {DEVICE_TABLE_NAME} TYPE record({DEVICE_MANUFACTURER_TABLE_NAME});
                DEFINE FIELD classification ON TABLE {DEVICE_TABLE_NAME} TYPE record({DEVICE_CLASSIFICATION_TABLE_NAME});
                DEFINE FIELD extension ON TABLE {DEVICE_TABLE_NAME} TYPE record({EXTENSION_TABLE_NAME});
                DEFINE FIELD primary_model_identifiers ON TABLE {DEVICE_TABLE_NAME} TYPE array<string>;
                DEFINE FIELD extended_model_identifiers ON TABLE {DEVICE_TABLE_NAME} TYPE array<string>;
                ",
            ))
            .await
            .unwrap_or_else(|_| {
                error!("Failed to set up database tables/schema.");
                stop(4);
            });

        Ok(())
    }

    /// Sets up IDs for "built-in" manufacturers and device classifications.
    pub async fn add_builtins(&self) -> anyhow::Result<()> {
        use surrealdb::{error::Api, Error};
        info!("Setting up reserved/built-in items...");
        match self.load_extension(InventoryExtension::builtin()).await {
            Ok(_) => Ok(()),
            Err(e) => match e {
                Error::Db(e) => Err(e.into()),
                Error::Api(e) => match e {
                    Api::Query(s)
                        if s == "There was a problem with the database: Database record \
                        `extensions:builtin` already exists" =>
                    {
                        warn!(
                            "Cannot re-add built-in items because they have already been loaded."
                        );
                        Ok(())
                    }
                    _ => Err(e.into()),
                },
            },
        }
    }

    /// Deletes all items from the database, but leaves the schema intact.
    /// Used for testing purposes.
    #[cfg(test)]
    #[allow(dead_code)]
    pub async fn clear(&self) {
        self.connection
            .delete::<Vec<GenericPullRecord>>(EXTENSION_TABLE_NAME)
            .await
            .unwrap();
        self.connection
            .delete::<Vec<GenericPullRecord>>(DEVICE_MANUFACTURER_TABLE_NAME)
            .await
            .unwrap();
        self.connection
            .delete::<Vec<GenericPullRecord>>(DEVICE_CLASSIFICATION_TABLE_NAME)
            .await
            .unwrap();
        self.connection
            .delete::<Vec<GenericPullRecord>>(DEVICE_TABLE_NAME)
            .await
            .unwrap();
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
    pub async fn load_extension(&self, extension: InventoryExtension) -> surrealdb::Result<()> {
        self.connection
            .create::<Vec<GenericPullRecord>>(EXTENSION_TABLE_NAME)
            .content(InventoryExtensionMetadataPushRecord::from(
                &extension.metadata,
            ))
            .await?;

        let mut futures = Vec::new();
        for classification in extension.device_classifications {
            futures.push(self.add_device_classification(classification));
        }
        future::join_all(futures).await;

        let mut futures = Vec::new();
        for manufacturer in extension.device_manufacturers {
            futures.push(self.add_device_manufacturer(manufacturer));
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
        future::join_all(futures).await;

        Ok(())
    }

    /// Removes an extension and its contents from the database.
    pub async fn unload_extension(
        &self,
        extension_id: &InventoryExtensionUniqueID,
    ) -> anyhow::Result<()> {
        if extension_id.unnamespaced() == "builtin" {
            return Err(anyhow::anyhow!("Cannot unload built-in extension"));
        }

        self.connection
            .query(&format!(
                "
                DELETE {DEVICE_MANUFACTURER_TABLE_NAME} WHERE extensions = [\"{0}\"];
                DELETE {DEVICE_CLASSIFICATION_TABLE_NAME} WHERE extensions = [\"{0}\"];
                DELETE {DEVICE_TABLE_NAME} WHERE extension = \"{0}\";
                DELETE {EXTENSION_TABLE_NAME} WHERE id = \"{0}\";
                ",
                extension_id.namespaced()
            ))
            .await?;

        Ok(())
    }

    /// Removes the extension corresponding to the ID of the given extension, and loads the given
    /// extension in its place.
    pub async fn reload_extension(&self, extension: InventoryExtension) -> anyhow::Result<()> {
        self.unload_extension(&extension.metadata.id).await?;
        self.load_extension(extension).await?;
        Ok(())
    }

    /// Lists all currently-loaded extensions in the database.
    pub async fn list_extensions(&self) -> anyhow::Result<Vec<InventoryExtensionMetadata>> {
        let pull_records = self
            .connection
            .select::<Vec<InventoryExtensionMetadataPullRecord>>(EXTENSION_TABLE_NAME)
            .await?;

        let mut extensions = Vec::new();
        for record in pull_records {
            extensions.push(InventoryExtensionMetadata::try_from(record)?);
        }

        Ok(extensions)
    }

    /// Lists all the device manufacturers in the database.
    #[allow(dead_code)]
    pub async fn list_device_manufacturers(&self) -> anyhow::Result<Vec<DeviceManufacturer>> {
        let pull_records = self
            .connection
            .select::<Vec<DeviceManufacturerPullRecord>>(DEVICE_MANUFACTURER_TABLE_NAME)
            .await?;

        let mut manufacturers = Vec::new();
        for record in pull_records {
            manufacturers.push(DeviceManufacturer::try_from(record)?);
        }

        Ok(manufacturers)
    }

    /// Lists all the device classifications in the database.
    #[allow(dead_code)]
    pub async fn list_device_classifications(&self) -> anyhow::Result<Vec<DeviceClassification>> {
        let pull_records = self
            .connection
            .select::<Vec<DeviceClassificationPullRecord>>(DEVICE_CLASSIFICATION_TABLE_NAME)
            .await?;

        let mut classifications = Vec::new();
        for record in pull_records {
            classifications.push(DeviceClassification::try_from(record)?);
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

    /// Adds a deivice manufacturer to the database, merging it with an existing record if needed.
    async fn add_device_manufacturer(
        &self,
        mut manufacturer: DeviceManufacturer,
    ) -> anyhow::Result<()> {
        if let Some(existing_record) = self.get_device_manufacturer(&manufacturer.id).await? {
            manufacturer.merge(existing_record.try_into()?);
        }

        self.connection
            .create::<Vec<GenericPullRecord>>(DEVICE_MANUFACTURER_TABLE_NAME)
            .content(DeviceManufacturerPushRecord::from(&manufacturer))
            .await?;

        Ok(())
    }

    /// Adds a device classification to the database, merging it with an existing record if needed.
    async fn add_device_classification(
        &self,
        mut classification: DeviceClassification,
    ) -> anyhow::Result<()> {
        if let Some(existing_record) = self.get_device_classification(&classification.id).await? {
            classification.merge(existing_record.try_into()?);
        }

        self.connection
            .create::<Vec<GenericPullRecord>>(DEVICE_CLASSIFICATION_TABLE_NAME)
            .content(DeviceClassificationPushRecord::from(&classification))
            .await?;

        Ok(())
    }

    // ? Can this be combined with `get_device_classification()` into a single function?
    /// Gets a device manufacturer from the database, if it exists.
    async fn get_device_manufacturer(
        &self,
        id: &DeviceManufacturerUniqueID,
    ) -> anyhow::Result<Option<DeviceManufacturerPullRecord>> {
        Ok(self
            .connection
            .select::<Option<DeviceManufacturerPullRecord>>((
                DEVICE_MANUFACTURER_TABLE_NAME,
                id.unnamespaced(),
            ))
            .await?)
    }

    /// Gets a device classification from the database, if it exists.
    async fn get_device_classification(
        &self,
        id: &DeviceClassificationUniqueID,
    ) -> anyhow::Result<Option<DeviceClassificationPullRecord>> {
        Ok(self
            .connection
            .select::<Option<DeviceClassificationPullRecord>>((
                DEVICE_CLASSIFICATION_TABLE_NAME,
                id.unnamespaced(),
            ))
            .await?)
    }

    /// Checks that the database only contains the given extension and its contents.
    /// Used for testing purposes.
    #[cfg(test)]
    #[allow(dead_code)]
    pub async fn only_contains(&self, extension: &InventoryExtension) {
        let loaded_extensions = self.list_extensions().await.unwrap();
        let loaded_device_manufacturers = self.list_device_manufacturers().await.unwrap();
        let loaded_device_classifications = self.list_device_classifications().await.unwrap();
        let loaded_devices = self.list_devices().await.unwrap();

        assert_eq!(loaded_extensions.len(), 1);
        assert_eq!(loaded_extensions[0], extension.metadata);

        assert_eq!(
            loaded_device_manufacturers.len(),
            extension.device_manufacturers.len()
        );
        assert_eq!(
            loaded_device_classifications.len(),
            extension.device_classifications.len()
        );
        assert_eq!(loaded_devices.len(), extension.devices.len());

        for manufacturer in &extension.device_manufacturers {
            assert!(loaded_device_manufacturers.contains(manufacturer));
        }

        for classification in &extension.device_classifications {
            assert!(loaded_device_classifications.contains(classification));
        }

        for device in &extension.devices {
            assert!(loaded_devices.contains(device));
        }
    }
}
