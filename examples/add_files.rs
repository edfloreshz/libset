use anyhow::Result;
use libset;
use libset::project::Project;
use libset::format::FileFormat;
use libset::new_file;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, Default, Eq, PartialEq)]
pub struct Settings {
    pub dark_mode_enabled: bool,
}

fn main() -> Result<()> {
    let settings = Settings { dark_mode_enabled: true };

    Project::new("com", "example", "App")
        .author("Eduardo Flores")
        .about("Example.")
        .version("0.1.1")
        .add_files(&[
            new_file!("settings").set_format(FileFormat::JSON).set_content(&settings)?,
            new_file!("settings").set_format(FileFormat::Plain).set_text("testing plain text")?,
        ])?;
    Ok(())
}
