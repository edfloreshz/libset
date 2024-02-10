use std::path::{Path, PathBuf};

use serde::{de::DeserializeOwned, Serialize};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("'{0}' is not a valid application name, avoid using . or .. .")]
    InvalidName(String),
    #[error("Filesystem error: {0}")]
    Io(std::io::Error),
    #[error("Config directory not found")]
    NoConfigDirectory,
    #[error("Failed to get key {0} : {1}")]
    GetKey(String, std::io::Error),
}

impl From<std::io::Error> for Error {
    fn from(f: std::io::Error) -> Self {
        Self::Io(f)
    }
}

pub trait Get {
    /// Get a configuration value
    fn get<T: DeserializeOwned>(&self, key: &str) -> Result<T, Error>;
}

pub trait Set {
    /// Set a configuration value
    fn set<T: Serialize>(&self, key: &str, value: T) -> Result<(), Error>;
}

pub struct Config {
    system_path: Option<PathBuf>,
    user_path: PathBuf,
}

/// Check that the name is relative and doesn't contain . or ..
fn sanitize_name(name: &str) -> Result<&Path, Error> {
    let path = Path::new(name);
    if path
        .components()
        .all(|x| matches!(x, std::path::Component::Normal(_)))
    {
        Ok(path)
    } else {
        Err(Error::InvalidName(name.to_owned()))
    }
}

impl Config {
    pub fn new(name: &str, version: u64) -> Result<Self, Error> {
        // Look for [name]/v[version]
        let path = sanitize_name(name)?.join(format!("v{}", version));

        // Search data file, which provides default (e.g. /usr/share)
        #[cfg(unix)]
        let system_path = xdg::BaseDirectories::with_prefix("cosmic")
            .map_err(std::io::Error::from)?
            .find_data_file(&path);

        #[cfg(windows)]
        let system_path =
            known_folders::get_known_folder_path(known_folders::KnownFolder::ProgramFilesCommon)
                .map(|x| x.join(&path));

        // Get libcosmic user configuration directory
        let user_path = dirs::config_dir().ok_or(Error::NoConfigDirectory)?;

        Ok(Self {
            system_path,
            user_path,
        })
    }
}

// /// This is where the Config data structure lives, most of that you do will be thanks to this struct.
// pub mod config;
// /// Contains the Element data structure.
// pub mod element;
// /// Contains the ElementFormat and FileType enums.
// pub mod format;
// /// Contains macros to facilitate the creation of new instances.
// mod macros;
// /// Contains the home and data paths.
// pub mod routes;
