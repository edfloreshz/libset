use anyhow::Result;
use libdmd::config::Config;
use libdmd::{directory, fi};

fn main() -> Result<()> {
    let config = Config::new("devmode")
        .author("Eduardo Flores")
        .about("Development management app.")
        .version("0.1.1")
        .add(directory!("config").child(fi!("config.toml")))
        .add(directory!("logs"))
        .add(directory!("paths").child(fi!("devpaths")));
    println!("{:#?}", config);
    Ok(())
}
