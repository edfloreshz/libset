use anyhow::Result;
use libset;
use libset::config::Config;

fn main() -> Result<()> {
    Config::new("devmode")
        .author("Eduardo Flores")
        .about("Development management app.")
        .version("0.1.1")
        .write()?;
    Ok(())
}
