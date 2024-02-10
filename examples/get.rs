use libset::{Config, Error};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct Settings {
    title: String,
}

fn main() -> Result<(), Error> {
    let config = Config::new("org.example.Demo", 1, None)?;
    let demo: Settings = config.get_toml("config")?;
    println!("{demo:?}");
    Ok(())
}
