use libset::{Config, Error};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct Settings {
    title: String,
}

fn main() -> Result<(), Error> {
    let config = Config::new("org.example.Demo", 1, None)?;
    let settings: Settings = config.get_json("settings")?;
    println!("{settings:?}");
    Ok(())
}
