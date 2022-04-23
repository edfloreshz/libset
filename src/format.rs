use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum FileType {
    TOML,
    JSON,
}

impl Default for FileType {
    fn default() -> Self {
        Self::TOML
    }
}
