use std::path::PathBuf;

pub fn data() -> PathBuf {
    dirs::data_dir().expect("Data directory not present.")
}

pub fn home() -> PathBuf {
    dirs::home_dir().expect("Home directory not present.")
}
