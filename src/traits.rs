use serde::{de::DeserializeOwned, Serialize};

use crate::{utils::FileType, Error};

pub trait Get {
    /// Get a configuration value
    fn get<T: DeserializeOwned>(&self, key: &str, file_type: FileType) -> Result<T, Error>;
}

pub trait Set {
    /// Set a configuration value
    fn set<T: Serialize>(&self, key: &str, value: T, file_type: FileType) -> Result<(), Error>;
}
