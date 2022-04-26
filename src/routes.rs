use std::path::PathBuf;

/// Returns the data directory of the current file system.
pub fn data() -> PathBuf {
    dirs::data_dir().expect("Data directory not present.")
}

/// Returns the home directory of the current file system.
pub fn home() -> PathBuf {
    dirs::home_dir().expect("Home directory not present.")
}
