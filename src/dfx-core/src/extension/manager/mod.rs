use crate::config::cache::get_cache_path_for_version;
use crate::error::extension::{
    GetExtensionBinaryError, ListInstalledExtensionsError, LoadExtensionManifestsError,
    NewExtensionManagerError,
};
use crate::extension::{
    installed::{InstalledExtensionList, InstalledExtensionManifests},
    manifest::ExtensionManifest,
};
pub use install::InstallOutcome;
use semver::Version;
use std::collections::HashMap;
use std::path::PathBuf;

mod execute;
mod install;
mod uninstall;

pub struct ExtensionManager {
    pub dir: PathBuf,
    pub dfx_version: Version,
}

impl ExtensionManager {
    pub fn new(version: &Version) -> Result<Self, NewExtensionManagerError> {
        let extensions_dir = get_cache_path_for_version(&version.to_string())?.join("extensions");

        Ok(Self {
            dir: extensions_dir,
            dfx_version: version.clone(),
        })
    }

    pub fn get_extension_directory(&self, extension_name: &str) -> PathBuf {
        self.dir.join(extension_name)
    }

    pub fn get_extension_binary(
        &self,
        extension_name: &str,
    ) -> Result<std::process::Command, GetExtensionBinaryError> {
        let dir = self.get_extension_directory(extension_name);
        if !dir.exists() {
            return Err(GetExtensionBinaryError::ExtensionNotInstalled(
                extension_name.to_string(),
            ));
        }
        let bin = dir.join(extension_name);
        if !bin.exists() {
            Err(GetExtensionBinaryError::ExtensionBinaryDoesNotExist(bin))
        } else if !bin.is_file() {
            Err(GetExtensionBinaryError::ExtensionBinaryIsNotAFile(bin))
        } else {
            Ok(std::process::Command::new(bin))
        }
    }

    pub fn is_extension_installed(&self, extension_name: &str) -> bool {
        self.get_extension_directory(extension_name).exists()
    }

    pub fn list_installed_extensions(
        &self,
    ) -> Result<InstalledExtensionList, ListInstalledExtensionsError> {
        if !self.dir.exists() {
            return Ok(vec![]);
        }
        let dir_content = crate::fs::read_dir(&self.dir)?;

        let extensions = dir_content
            .filter_map(|v| {
                let dir_entry = v.ok()?;
                if dir_entry.file_type().map_or(false, |e| e.is_dir())
                    && !dir_entry.file_name().to_str()?.starts_with(".tmp")
                {
                    let name = dir_entry.file_name().to_string_lossy().to_string();
                    Some(name)
                } else {
                    None
                }
            })
            .collect();
        Ok(extensions)
    }

    pub fn load_installed_extension_manifests(
        &self,
    ) -> Result<InstalledExtensionManifests, LoadExtensionManifestsError> {
        let manifests = self
            .list_installed_extensions()?
            .into_iter()
            .map(|name| {
                ExtensionManifest::load(&name, &self.dir)
                    .map(|manifest| (manifest.name.clone(), manifest))
            })
            .collect::<Result<HashMap<_, _>, _>>()?;
        Ok(InstalledExtensionManifests(manifests))
    }
}
