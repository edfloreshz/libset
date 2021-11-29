use serde::{Serialize, Deserialize};
use anyhow::Result;
use libdmd::utils::config::config::Config;
use libdmd::utils::config::directory::Directory;
use libdmd::utils::config::file::File;
use libdmd::utils::config::format::FileFormat;
use libdmd::utils::config::format::FileFormat::TOML;

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

    let mut config = Config::new()
        .project("devmode")
        .dir(
            Directory::new()
                .name("config")
                .file(
                    File::new()
                        .name("config")
                        .format(FileFormat::TOML)
                        .data(&options)?,
                ),
        )
        .dir(
            Directory::new().name("logs")
                .file(
                    File::new()
                        .name("logs")
                        .format(FileFormat::TOML)
                        .data(&log)?,
                )
        )
        .dir(Directory::new().name("paths"));
    config.build()?;
    println!("{:?}", config.current());
    println!("{:?}", Config::get::<Log>("devmode/logs/logs.toml", TOML));
    Ok(())
}
