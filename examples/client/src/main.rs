use anyhow::Result;
use libdmd::utils::config::*;
use libdmd::{dir, fi};

fn main() -> Result<()> {
    let config = Config::new("devmode")
        .author("Eduardo Flores")
        .about("Development management app.")
        .version("0.1.1")
        .add(dir!("config").child(fi!("config.toml")))
        .add(dir!("logs"))
        .add(dir!("paths").child(fi!("devpaths")));
    println!("{:#?}", config);
    Ok(())
}
