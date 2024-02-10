use std::{io::Write, path::PathBuf};

use serde::{de::DeserializeOwned, Serialize};
use tracing::info;

mod error;
mod traits;
mod utils;

pub use error::Error;
pub use traits::{Get, Set};
use utils::sanitize_name;
pub use utils::FileType;

pub struct Config {
    path: PathBuf,
}

impl Config {
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

    pub fn has_key(&self, key: &str, file_type: &FileType) -> bool {
        self.path.join(format!("{key}.{file_type}")).exists()
    }

    pub fn path(&self, key: &str, file_type: &FileType) -> Result<PathBuf, Error> {
        let path = self
            .path
            .join(sanitize_name(&format!("{key}.{file_type}"))?);
        info!("Found key {}.", key);
        Ok(path)
    }
}

impl Get for Config {
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
    fn set<T: Serialize>(&self, key: &str, value: T, file_type: FileType) -> Result<(), Error> {
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
