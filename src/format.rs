use serde::{Deserialize, Serialize};

/// Helper enum to indicate the supported file types.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
pub enum FileFormat {
    /// Tom's Obvious, Minimal Language.
    ///
    /// <https://github.com/toml-lang/toml>
    TOML,
    /// JavaScript Object Notation.
    ///
    /// <https://www.json.org/json-en.html>
    JSON,
}

impl Default for FileFormat {
    fn default() -> Self {
        Self::TOML
    }
}
