use libset::{Config, Error};

fn main() -> Result<(), Error> {
    let config = Config::new("org.example.Demo", 1, None)?;
    config.clean()?;
    Ok(())
}
