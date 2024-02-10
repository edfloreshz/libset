use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("'{0}' is not a valid application name, avoid using . or .. .")]
    InvalidName(String),
    #[error("Failed to write to file: {0}")]
    Write(atomicwrites::Error<std::io::Error>),
    #[error("Filesystem error: {0}")]
    Io(std::io::Error),
    #[error("Config directory not found")]
    NoConfigDirectory,
    #[error("Failed to get key {0} : {1}")]
    GetKey(String, std::io::Error),
    #[error("Failed to parse ron file: {0}")]
    Ron(ron::Error),
    #[error("Failed to parse ron file: {0}")]
    RonSpanned(ron::error::SpannedError),
    #[error("Failed to parse json file: {0}")]
    Json(serde_json::Error),
    #[error("Failed to serialize toml file: {0}")]
    TomlSerialize(toml::ser::Error),
    #[error("Failed to deserialize toml file: {0}")]
    TomlDeserialize(toml::de::Error),
    #[error("An error ocurred: {0}")]
    String(String),
}

impl From<String> for Error {
    fn from(f: String) -> Self {
        Self::String(f)
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
