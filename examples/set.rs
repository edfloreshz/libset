use libset::{Config, Error};

fn main() -> Result<(), Error> {
    Config::new("../Test", 1).map(|_| ())
}
