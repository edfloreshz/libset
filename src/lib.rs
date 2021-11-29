use std::path::PathBuf;

pub mod utils;

pub fn data() -> PathBuf {
    dirs::data_dir().expect("Data directory not present.")
}

pub fn home() -> PathBuf {
    dirs::home_dir().expect("Home directory not present.")
}
