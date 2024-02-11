//! # Libset
//!
//! Libset is a versatile library designed to simplify the storage, management, and retrieval of
//! configuration files. It provides seamless support for various file formats, allowing you to
//! effortlessly handle your configuration needs.
//!
//! Managing configuration files often involves repetitive tasks such as file handling, reading, writing,
//! error handling, and serialization/deserialization. This complexity increases significantly when
//! supporting multiple file formats. Libset alleviates these challenges by offering a user-friendly API,
//! freeing you to concentrate on more important aspects of your project.
//!
//! ## Design
//! Libset is engineered to seamlessly support a range of file formats, including:
//!
//! * [`JSON`](https://www.json.org/json-en.html) - JavaScript Object Notation
//! * [`TOML`](https://toml.io/en/) - Tom's Obvious Minimal Language
//! * [`RON`](https://github.com/ron-rs/ron) - Rusty Object Notation
//!
//! ## Features
//! By default, Libset enables JSON format support. Additional formats can be activated using feature flags:
//!
//! * `json` - Seamlessly interact with JSON files.
//! * `toml` - Effortlessly modify TOML files.
//! * `ron` - Easily retreive RON files.
//!
//! ## Additional Benefits

//! In addition to its robust file format support and user-friendly interface, Libset offers:
//! - **Performance**: Optimized for efficiency, ensuring swift processing of configuration operations.
//! - **Cross-Platform Compatibility**: Works seamlessly across different operating systems, enhancing flexibility in deployment.
//! - **Documentation**: Comprehensive documentation and examples make integration and usage straightforward for developers of all levels.

use std::{io::Write, path::PathBuf};

use serde::{de::DeserializeOwned, Serialize};
use tracing::info;

mod error;
mod traits;
mod utils;

pub use error::Error;
use traits::{Get, Set};
use utils::sanitize_name;
pub use utils::FileType;

/// Represents a configuration object.
///
/// Start by creating a new `Config` object:
/// ```
/// let config = Config::new("org.example.Demo", 1, None)?;
/// ```
/// Provide an application name, a version and optionally a prefix, then, a new directory will be added
/// to your filesystem, this is where all the created files will be stored in.
///
/// ### Write a file.
/// ```rust
/// let config = Config::new("org.example.Demo", 1, None)?;
/// config.set_json("colors", json!({ "accent": "#7a7af9" }))?;
/// ```
/// > This wil store the file here: `$HOME/.config/org.example.Demo/v1/colors.json`
///
/// ### Get a file.
/// ```rust
/// #[derive(Debug, Serialize, Deserialize)]
/// struct Colors { accent: String }
/// let settings: Colors = config.get_json("colors")?;
/// ```
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
    /// * `scope` - An optional scope for the application.
    ///
    /// # Returns
    ///
    /// A `Result` containing the new `Config` object or an `Error` if an error occurred.
    pub fn new(name: &str, version: u64, scope: Option<&str>) -> Result<Self, Error> {
        let main_path = sanitize_name(name)?.join(format!("v{}", version));

        let user_path = dirs::config_dir().ok_or(Error::NoConfigDirectory)?;

        let config_path = if let Some(scope) = scope {
            let scope = sanitize_name(scope)?;
            user_path.join(main_path).join(scope)
        } else {
            user_path.join(main_path)
        };

        std::fs::create_dir_all(&config_path)?;

        Ok(Self { path: config_path })
    }

    /// Determines if a plain file with the given key is present in the filesystem.
    ///
    /// # Arguments
    ///
    /// * `key` - The key used to store the file.
    ///
    /// # Returns
    ///
    /// `true` if the plain file exists, `false` otherwise.
    pub fn has_plain(&self, key: &str) -> bool {
        self.path.join(key).exists()
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
    #[cfg(feature = "toml")]
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
    #[cfg(feature = "json")]
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
    #[cfg(feature = "ron")]
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
    #[cfg(feature = "toml")]
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
    #[cfg(feature = "json")]
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
    #[cfg(feature = "ron")]
    pub fn get_ron<T: DeserializeOwned>(&self, key: &str) -> Result<T, Error> {
        self.get(key, FileType::Ron)
    }

    /// Gets the content of a plain file with the given key.
    ///
    /// # Arguments
    ///
    /// * `key` - The key used to store the file.
    ///
    /// # Returns
    ///
    /// A `Result` containing the value or an `Error` if an error occurred.
    pub fn get_plain(&self, key: &str) -> Result<String, Error> {
        std::fs::read_to_string(self.path.join(key))
            .map_err(|err| Error::GetKey(key.to_string(), err))
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
    #[cfg(feature = "toml")]
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
    #[cfg(feature = "json")]
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
    #[cfg(feature = "ron")]
    pub fn set_ron<T: Serialize>(&self, key: &str, value: T) -> Result<(), Error> {
        self.set(key, FileType::Ron, value)
    }

    /// Sets the content of a plain file with the given key.
    ///
    /// # Arguments
    ///
    /// * `key` - The key used to store the file.
    /// * `value` - String to write.
    ///
    /// # Returns
    ///
    /// A `Result` indicating success or an `Error` if an error occurred.
    pub fn set_plain(&self, key: &str, value: impl ToString) -> Result<(), Error> {
        let key_path = self.path.join(key);
        atomicwrites::AtomicFile::new(&key_path, atomicwrites::OverwriteBehavior::AllowOverwrite)
            .write(|file| file.write_all(value.to_string().as_bytes()))?;
        Ok(())
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
    pub fn path(&self, key: &str, file_type: FileType) -> Result<PathBuf, Error> {
        let name = if FileType::Plain == file_type {
            key.to_string()
        } else {
            format!("{key}.{file_type}")
        };
        let path = self.path.join(sanitize_name(&name)?);
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
        let key_path = self.path(key, file_type)?;
        let data = std::fs::read_to_string(&key_path)
            .map_err(|err| Error::GetKey(key.to_string(), err))?;

        let t = match file_type {
            #[cfg(feature = "toml")]
            FileType::Toml => toml::from_str(&data)?,
            #[cfg(feature = "json")]
            FileType::Json => serde_json::from_str(&data)?,
            #[cfg(feature = "ron")]
            FileType::Ron => ron::from_str(&data)?,
            FileType::Plain => unreachable!("Never get plain text with get method."),
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
        let key_path = self.path(key, file_type)?;
        let data = match file_type {
            #[cfg(feature = "toml")]
            FileType::Toml => toml::to_string_pretty(&value)?,
            #[cfg(feature = "json")]
            FileType::Json => serde_json::to_string_pretty(&value)?,
            #[cfg(feature = "ron")]
            FileType::Ron => ron::ser::to_string_pretty(&value, ron::ser::PrettyConfig::new())?,
            FileType::Plain => unreachable!("Never get plain text with get method."),
        };
        atomicwrites::AtomicFile::new(&key_path, atomicwrites::OverwriteBehavior::AllowOverwrite)
            .write(|file| file.write_all(data.as_bytes()))?;
        info!("File written to {}.", key_path.display());
        Ok(())
    }
}
