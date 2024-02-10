use libset::{Config, Error, FileType, Get};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct Settings {
    title: String,
}

fn main() -> Result<(), Error> {
    let config = Config::new("org.example.Demo", 1, None)?;
    let demo: Settings = config.get("config", FileType::Toml)?;
    println!("{demo:?}");
    Ok(())
}
