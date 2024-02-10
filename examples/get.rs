use libset::{Config, Error};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct Colors {
    accent: String,
}

fn main() -> Result<(), Error> {
    let config = Config::new("org.example.Demo", 1, None)?;
    let colors: Colors = config.get_json("colors")?;
    println!("{colors:?}");
    Ok(())
}
