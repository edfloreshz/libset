use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum FileType {
    TOML,
    JSON,
}

impl FileType {
    pub fn ext(&self, file: &str) -> String {
        match self {
            FileType::TOML => format!("{}{}", file, ".toml"),
            FileType::JSON => format!("{}{}", file, ".json"),
        }
    }
}

impl Default for FileType {
    fn default() -> Self {
        Self::TOML
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ElementFormat {
    Directory,
    File,
}