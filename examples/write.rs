use libset;
use libset::config::Config;
use anyhow::Result;

fn main() -> Result<()> {
    Config::new("devmode")
        .author("Eduardo Flores")
        .about("Development management app.")
        .version("0.1.1")
        .write()?;
    Ok(())
}