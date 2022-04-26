use anyhow::Result;
use libset::config::Config;
use libset::{directory, fi};

fn main() -> Result<()> {
    let config = Config::new("devmode")
        .author("Eduardo Flores")
        .about("Development management app.")
        .version("0.1.1")
        .add(directory!("config").add_child(fi!("config.toml")))
        .add(directory!("logs"))
        .add(directory!("paths").add_child(fi!("devpaths")));
    println!("{:#?}", config);
    Ok(())
}
