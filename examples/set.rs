use libset::{Config, Error};
use serde::{Deserialize, Serialize};
use serde_json::json;

#[derive(Debug, Serialize, Deserialize)]
struct Settings {
    title: String,
}

fn main() -> Result<(), Error> {
    let config = Config::new("org.example.Demo", 1, None)?;
    config.set_json("colors", json!({ "accent": "#7a7af9" }))?;
    Ok(())
}