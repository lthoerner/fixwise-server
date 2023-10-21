use log::warn;
use semver::Version;

use super::{ExtensionID, InventoryExtension as Extension, Metadata};

/// Indicator that the manager encountered an error when staging an extension.
pub struct StageConflict {
    #[allow(dead_code)]
    id: ExtensionID,
}

impl StageConflict {
    pub fn new(metadata: &Metadata) -> Self {
        Self {
            id: metadata.id.clone(),
        }
    }
}

/// Indicator that the common name of a staged extension did not match its loaded counterpart.
#[derive(Debug, PartialEq, Eq)]
pub(super) struct NameChange {
    pub(super) loaded_name: String,
    pub(super) staged_name: String,
}

/// Indicator that the version of a staged extension did not match its loaded counterpart.
#[derive(Debug, PartialEq, Eq)]
pub(super) struct VersionChange {
    pub(super) loaded_version: Version,
    pub(super) staged_version: Version,
}

/// Indicator that the manager encountered an error when loading an extension.
#[derive(Debug, PartialEq, Eq)]
pub struct LoadConflict {
    pub(super) id: ExtensionID,
    pub(super) name_change: Option<NameChange>,
    pub(super) version_change: Option<VersionChange>,
}

impl LoadConflict {
    /// Checks whether a given staged extension conflicts with any of the given loaded extensions.
    /// If it does, the conflict is returned.
    // * Any staged extension can only logically have up to one conflict with a loaded
    // * extension, and vice versa, because of the following reasons:
    // * - Conflicts can only arise when a staged and a loaded extension share the same ID.
    // * - No two loaded extensions can have the same ID due to database constraints.
    // * - No two staged extensions can have the same ID because they are pre-filtered.
    pub fn new(
        staged_extension: &Extension,
        loaded_extensions: &mut Vec<Metadata>,
    ) -> Option<Self> {
        let staged_extension_metadata = &staged_extension.metadata;
        for (i, loaded_extension_metadata) in loaded_extensions.iter().enumerate() {
            // Check the difference between the loaded and staged extensions.
            let diff = ExtensionDiff::new(loaded_extension_metadata, staged_extension_metadata);

            // If the extensions have different IDs, move on to the next loaded extension.
            let Some(diff) = diff else {
                continue;
            };

            // Otherwise, determine the conflict.
            let conflict = LoadConflict {
                id: loaded_extension_metadata.id.clone(),
                name_change: if diff.is_name_change() {
                    Some(NameChange {
                        loaded_name: loaded_extension_metadata.common_name.clone(),
                        staged_name: staged_extension_metadata.common_name.clone(),
                    })
                } else {
                    None
                },
                version_change: if diff.is_update() || diff.is_downgrade() {
                    Some(VersionChange {
                        loaded_version: loaded_extension_metadata.version.clone(),
                        staged_version: staged_extension_metadata.version.clone(),
                    })
                } else {
                    None
                },
            };

            // Skip the conflicting extension in subsequent conflict checks for optimization.
            loaded_extensions.remove(i);
            return Some(conflict);
        }

        None
    }

    /// Logs the appropriate message for a conflict.
    pub fn log(&self, load_override: bool) {
        let id_string = self.id.to_non_namespaced_string();

        if let Some(name_change) = &self.name_change {
            warn!(
                "Loaded and staged extension with ID '{}' have conflicting common names '{}' and \
                '{}'.",
                &id_string, &name_change.loaded_name, &name_change.staged_name
            );
        }

        if load_override {
            warn!(
                "Reloading extension '{}' due to a load override.",
                &id_string
            );
        } else if let Some(version_change) = &self.version_change {
            if version_change.loaded_version < version_change.staged_version {
                warn!(
                    "Updating extension '{}' from v{} to v{}.",
                    &id_string, version_change.loaded_version, version_change.staged_version
                );
            } else {
                warn!(
                    "Skipping extension '{}' because a newer version is already loaded.",
                    &id_string
                );
            }
        } else {
            warn!(
                "Skipping extension '{}' because it is already loaded and its version has not been \
                changed.",
                &id_string
            );
        }
    }

    /// Checks whether the conflict indicates that the extension should be reloaded or skipped.
    /// Load override flag should be checked before calling this method.
    pub fn should_reload(&self) -> bool {
        if let Some(version_change) = &self.version_change {
            version_change.loaded_version < version_change.staged_version
        } else {
            false
        }
    }
}

/// The difference between the metadata of two extensions, used to determine conflicts.
/// Does not account for extension contents.
struct ExtensionDiff {
    same_common_name: bool,
    higher_version: Option<bool>,
}

impl ExtensionDiff {
    /// Generates a diff between the metadata of a loaded and a staged extension, returning `None`
    /// if the extensions do not have the same ID and are thus incomparable.
    fn new(extension_1: &Metadata, extension_2: &Metadata) -> Option<Self> {
        if extension_1.id != extension_2.id {
            return None;
        }

        let higher_version = if extension_1.version > extension_2.version {
            Some(true)
        } else {
            match extension_1.version < extension_2.version {
                true => Some(false),
                false => None,
            }
        };

        Some(Self {
            same_common_name: extension_1.common_name == extension_2.common_name,
            higher_version,
        })
    }

    /// Checks whether the loaded extension is being updated by the staged extension.
    fn is_update(&self) -> bool {
        self.same_common_name && self.higher_version == Some(true)
    }

    /// Checks whether the loaded extension is being downgraded by the staged extension.
    fn is_downgrade(&self) -> bool {
        self.same_common_name && self.higher_version == Some(false)
    }

    /// Checks whether the diff indicates that the extension name is being changed.
    fn is_name_change(&self) -> bool {
        !self.same_common_name
    }
}
