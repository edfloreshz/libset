use std::{fmt::Display, path::Path};

use tracing::error;

use crate::Error;

pub(crate) enum FileType {
    Toml,
    Json,
    Ron,
}

impl Display for FileType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            FileType::Toml => write!(f, "toml"),
            FileType::Json => write!(f, "json"),
            FileType::Ron => write!(f, "ron"),
        }
    }
}

/// Check that the name is relative.
///
/// # Arguments
///
/// * `name` - The name to be sanitized
///
/// # Returns
///
/// Returns a `Result` containing a reference to the sanitized `Path` if the name is valid,
/// otherwise returns an `Error` indicating the invalid name.
pub(crate) fn sanitize_name(name: &str) -> Result<&Path, Error> {
    let path = Path::new(name);
    if path
        .components()
        .all(|x| matches!(x, std::path::Component::Normal(_)))
    {
        Ok(path)
    } else {
        let error = Error::InvalidName(name.to_owned());
        error!("{}", error.to_string());
        Err(error)
    }
}
