use anyhow::Result;
use libset::config::Config;
use libset::element::Content;
use libset::format::FileFormat;
use libset::new_file;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, Default, Eq, PartialEq)]
pub struct Settings {
    pub host: String,
    pub owner: String,
    pub editor: String,
    pub workspaces: Vec<String>,
}

impl Content for Settings {}

fn main() -> Result<()> {
    let config = Config::new("devmode")
        .author("Eduardo Flores")
        .about("Development management app.")
        .version("0.1.1")
        .add(
            new_file!("settings.toml")
                .set_format(FileFormat::TOML)
                .set_content(Box::new(Settings::default())),
        )
        .add(new_file!("devpaths"))
        .write()?;
    println!("{:#?}", config);
    Ok(())
}
