use std::{io::Write, path::PathBuf};

use serde::{de::DeserializeOwned, Serialize};
use tracing::info;

mod error;
mod traits;
mod utils;

pub use error::Error;
use traits::{Get, Set};
use utils::sanitize_name;
use utils::FileType;

/// Represents a configuration object.
pub struct Config {
    path: PathBuf,
}

impl Config {
    /// Creates a new `Config` object.
    ///
    /// # Arguments
    ///
    /// * `name` - The name of the application.
    /// * `version` - The version of the configuration.
    /// * `prefix` - An optional prefix for the application.
    ///
    /// # Returns
    ///
    /// A `Result` containing the new `Config` object or an `Error` if an error occurred.
    pub fn new(name: &str, version: u64, prefix: Option<&str>) -> Result<Self, Error> {
        let path = sanitize_name(name)?.join(format!("v{}", version));

        let user_path = dirs::config_dir().ok_or(Error::NoConfigDirectory)?;

        let config_path = if let Some(prefix) = prefix {
            let prefix = sanitize_name(prefix)?;
            user_path.join(prefix).join(path)
        } else {
            user_path.join(path)
        };

        std::fs::create_dir_all(&config_path)?;

        Ok(Self { path: config_path })
    }

    /// Determines if a toml file with the given key is present in the filesystem.
    ///
    /// # Arguments
    ///
    /// * `key` - The key used to store the file.
    ///
    /// # Returns
    ///
    /// `true` if the toml file exists, `false` otherwise.
    pub fn has_toml(&self, key: &str) -> bool {
        self.path.join(format!("{key}.{}", FileType::Toml)).exists()
    }

    /// Determines if a json file with the given key is present in the filesystem.
    ///
    /// # Arguments
    ///
    /// * `key` - The key used to store the file.
    ///
    /// # Returns
    ///
    /// `true` if the json file exists, `false` otherwise.
    pub fn has_json(&self, key: &str) -> bool {
        self.path.join(format!("{key}.{}", FileType::Json)).exists()
    }

    /// Determines if a ron file with the given key is present in the filesystem.
    ///
    /// # Arguments
    ///
    /// * `key` - The key used to store the file.
    ///
    /// # Returns
    ///
    /// `true` if the ron file exists, `false` otherwise.
    pub fn has_ron(&self, key: &str) -> bool {
        self.path.join(format!("{key}.{}", FileType::Ron)).exists()
    }

    /// Gets the content of a toml file with the given key and deserializes it into a type.
    ///
    /// # Arguments
    ///
    /// * `key` - The key used to store the file.
    ///
    /// # Returns
    ///
    /// A `Result` containing the deserialized value or an `Error` if an error occurred.
    pub fn get_toml<T: DeserializeOwned>(&self, key: &str) -> Result<T, Error> {
        self.get(key, FileType::Toml)
    }

    /// Gets the content of a json file with the given key and deserializes it into a type.
    ///
    /// # Arguments
    ///
    /// * `key` - The key used to store the file.
    ///
    /// # Returns
    ///
    /// A `Result` containing the deserialized value or an `Error` if an error occurred.
    pub fn get_json<T: DeserializeOwned>(&self, key: &str) -> Result<T, Error> {
        self.get(key, FileType::Json)
    }

    /// Gets the content of a ron file with the given key and deserializes it into a type.
    ///
    /// # Arguments
    ///
    /// * `key` - The key used to store the file.
    ///
    /// # Returns
    ///
    /// A `Result` containing the deserialized value or an `Error` if an error occurred.
    pub fn get_ron<T: DeserializeOwned>(&self, key: &str) -> Result<T, Error> {
        self.get(key, FileType::Ron)
    }

    /// Sets the content of a toml file with the given key and serializes the value.
    ///
    /// # Arguments
    ///
    /// * `key` - The key used to store the file.
    /// * `value` - The value to be serialized and stored.
    ///
    /// # Returns
    ///
    /// A `Result` indicating success or an `Error` if an error occurred.
    pub fn set_toml<T: Serialize>(&self, key: &str, value: T) -> Result<(), Error> {
        self.set(key, FileType::Toml, value)
    }

    /// Sets the content of a json file with the given key and serializes the value.
    ///
    /// # Arguments
    ///
    /// * `key` - The key used to store the file.
    /// * `value` - The value to be serialized and stored.
    ///
    /// # Returns
    ///
    /// A `Result` indicating success or an `Error` if an error occurred.
    pub fn set_json<T: Serialize>(&self, key: &str, value: T) -> Result<(), Error> {
        self.set(key, FileType::Json, value)
    }

    /// Sets the content of a ron file with the given key and serializes the value.
    ///
    /// # Arguments
    ///
    /// * `key` - The key used to store the file.
    /// * `value` - The value to be serialized and stored.
    ///
    /// # Returns
    ///
    /// A `Result` indicating success or an `Error` if an error occurred.
    pub fn set_ron<T: Serialize>(&self, key: &str, value: T) -> Result<(), Error> {
        self.set(key, FileType::Ron, value)
    }

    /// Given a key, returns the file path in the filesystem.
    ///
    /// # Arguments
    ///
    /// * `key` - The key used to store the file.
    /// * `file_type` - The file extension.
    ///
    /// # Returns
    ///
    /// A `Result` containing the file path or an `Error` if an error occurred.
    fn path(&self, key: &str, file_type: &FileType) -> Result<PathBuf, Error> {
        let path = self
            .path
            .join(sanitize_name(&format!("{key}.{file_type}"))?);
        info!("Found key {}.", key);
        Ok(path)
    }
}

impl Get for Config {
    /// Given a key, obtains the file and parses it into a type.
    ///
    /// # Arguments
    ///
    /// * `key` - The key used to store the file.
    /// * `file_type` - The file extension.
    ///
    /// # Returns
    ///
    /// A `Result` containing the deserialized value or an `Error` if an error occurred.
    fn get<T: DeserializeOwned>(&self, key: &str, file_type: FileType) -> Result<T, Error> {
        let key_path = self.path(key, &file_type)?;
        let data = std::fs::read_to_string(&key_path)
            .map_err(|err| Error::GetKey(key.to_string(), err))?;

        let t = match file_type {
            FileType::Toml => toml::from_str(&data)?,
            FileType::Json => serde_json::from_str(&data)?,
            FileType::Ron => ron::from_str(&data)?,
        };
        info!("Retrieved file from {}.", key_path.display());
        Ok(t)
    }
}

impl Set for Config {
    /// Given a key, saves the serialized value to the file.
    ///
    /// # Arguments
    ///
    /// * `key` - The key used to store the file.
    /// * `file_type` - The file extension.
    ///
    /// # Returns
    ///
    /// A `Result` containing the deserialized value or an `Error` if an error occurred.
    fn set<T: Serialize>(&self, key: &str, file_type: FileType, value: T) -> Result<(), Error> {
        let key_path = self.path(key, &file_type)?;
        let data = match file_type {
            FileType::Toml => toml::to_string_pretty(&value)?,
            FileType::Json => serde_json::to_string_pretty(&value)?,
            FileType::Ron => ron::ser::to_string_pretty(&value, ron::ser::PrettyConfig::new())?,
        };
        atomicwrites::AtomicFile::new(&key_path, atomicwrites::OverwriteBehavior::AllowOverwrite)
            .write(|file| file.write_all(data.as_bytes()))?;
        info!("File written to {}.", key_path.display());
        Ok(())
    }
}
