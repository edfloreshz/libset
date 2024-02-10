use serde::{de::DeserializeOwned, Serialize};

use crate::{utils::FileType, Error};

pub(crate) trait Get {
    /// Get a configuration value.
    ///
    /// Retrieves a configuration value of type `T` associated with the given `key` and `file_type`.
    /// Returns a `Result` containing the retrieved value on success, or an `Error` on failure.
    fn get<T: DeserializeOwned>(&self, key: &str, file_type: FileType) -> Result<T, Error>;
}

pub(crate) trait Set {
    /// Set a configuration value.
    ///
    /// Sets the configuration value of type `T` associated with the given `key` and `file_type` to the provided `value`.
    /// Returns a `Result` indicating success or failure.
    fn set<T: Serialize>(&self, key: &str, file_type: FileType, value: T) -> Result<(), Error>;
}
