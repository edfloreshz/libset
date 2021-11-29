use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum FileFormat {
    TOML,
    JSON,
}

impl FileFormat {
    pub fn ext(&self, file: &str) -> String {
        match self {
            FileFormat::TOML => format!("{}{}", file, ".toml"),
            FileFormat::JSON => format!("{}{}", file, ".json"),
        }
    }
}

impl Default for FileFormat {
    fn default() -> Self {
        Self::TOML
    }
}
