use thiserror::Error;

/// Custom error type for the library.
#[derive(Debug, Error)]
pub enum Error {
    /// Represents an invalid application name.
    #[error("'{0}' is not a valid application name, avoid using . or .. .")]
    InvalidName(String),
    /// Represents a failure to write to a file.
    #[error("Failed to write to file: {0}")]
    Write(atomicwrites::Error<std::io::Error>),
    /// Represents a filesystem error.
    #[error("Filesystem error: {0}")]
    Io(std::io::Error),
    /// Represents a missing configuration directory.
    #[error("Config directory not found")]
    NoConfigDirectory,
    /// Represents a failure to get a key.
    #[error("Failed to get key {0} : {1}")]
    GetKey(String, std::io::Error),
    /// Represents a failure to parse a ron file.
    #[error("Failed to parse ron file: {0}")]
    Ron(ron::Error),
    /// Represents a failure to parse a ron file with span information.
    #[error("Failed to parse ron file: {0}")]
    RonSpanned(ron::error::SpannedError),
    /// Represents a failure to parse a json file.
    #[error("Failed to parse json file: {0}")]
    Json(serde_json::Error),
    /// Represents a failure to serialize a toml file.
    #[error("Failed to serialize toml file: {0}")]
    TomlSerialize(toml::ser::Error),
    /// Represents a failure to deserialize a toml file.
    #[error("Failed to deserialize toml file: {0}")]
    TomlDeserialize(toml::de::Error),
    /// Represents a generic string error.
    #[error("An error ocurred: {0}")]
    Generic(String),
}

impl From<String> for Error {
    fn from(f: String) -> Self {
        Self::Generic(f)
    }
}

impl From<atomicwrites::Error<std::io::Error>> for Error {
    fn from(f: atomicwrites::Error<std::io::Error>) -> Self {
        Self::Write(f)
    }
}

impl From<std::io::Error> for Error {
    fn from(f: std::io::Error) -> Self {
        Self::Io(f)
    }
}

impl From<ron::Error> for Error {
    fn from(f: ron::Error) -> Self {
        Self::Ron(f)
    }
}

impl From<ron::error::SpannedError> for Error {
    fn from(f: ron::error::SpannedError) -> Self {
        Self::RonSpanned(f)
    }
}

impl From<serde_json::Error> for Error {
    fn from(f: serde_json::Error) -> Self {
        Self::Json(f)
    }
}

impl From<toml::de::Error> for Error {
    fn from(f: toml::de::Error) -> Self {
        Self::TomlDeserialize(f)
    }
}

impl From<toml::ser::Error> for Error {
    fn from(f: toml::ser::Error) -> Self {
        Self::TomlSerialize(f)
    }
}
