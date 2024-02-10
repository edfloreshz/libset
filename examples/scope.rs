use libset::{Config, Error};
use serde::{Deserialize, Serialize};
use serde_json::json;

#[derive(Debug, Serialize, Deserialize)]
struct Colors {
    accent: String,
}

fn main() -> Result<(), Error> {
    let config = Config::new("org.example.Demo", 1, Some("appearance"))?;
    config.set_json("colors", json!({ "accent": "#7a7af9" }))?;
    let colors: Colors = config.get_json("colors")?;
    println!("{colors:?}");
    Ok(())
}
