use serde::{Serialize, Deserialize};
use anyhow::Result;
use libdmd::utils::config::config::ConfigBuilder;
use libdmd::utils::config::directory::DirectoryBuilder;
use libdmd::utils::config::file::FileBuilder;
use libdmd::utils::config::format::FileFormat;

#[derive(Serialize, Deserialize, Debug, Clone, Default, Eq, PartialEq)]
pub struct AppOptions {
    pub host: String,
    pub owner: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default, Eq, PartialEq)]
pub struct Log {
    pub message: String,
}

fn main() -> Result<()> {
    let options = AppOptions {
        host: "GitHub".to_string(),
        owner: "edfloreshz".to_string(),
    };
    let log = Log {
        message: "Logs".to_string()
    };

    let mut config = ConfigBuilder::new()
        .project("devmode")
        .dir(
            DirectoryBuilder::new()
                .name("config")
                .file(
                    FileBuilder::new()
                        .name("config")
                        .format(FileFormat::TOML)
                        .data(&options)?,
                ),
        )
        .dir(
            DirectoryBuilder::new().name("logs")
                .file(
                    FileBuilder::new()
                        .name("logs")
                        .format(FileFormat::TOML)
                        .data(&log)?,
                )
        )
        .dir(DirectoryBuilder::new().name("paths"));
    config.build()?;
    Ok(())
}
