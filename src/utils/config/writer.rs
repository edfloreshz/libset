
use anyhow::Result;
use serde::{Deserialize, Serialize, de::{DeserializeOwned}};
use serde_value::Value;

pub trait Writer {
    fn write() -> Result<()>;
}

#[derive(Serialize, Deserialize)]
pub struct Options {
    pub color: bool,
}

impl Writer for Options {
    fn write() -> Result<()> {
        todo!()
    }
}

/// Parse config file to a struct.
/// 
/// Eample: 
/// ```rust 
/// use libdmd::utils::config::writer::{parse, Options};
/// use anyhow::Result;
/// struct Config {
///     color: bool
/// }
/// 
/// fn main() -> Result<()> {
///     let config = parse<Config>("/Users/eduardo/Library/Application Support/devmode/config/config.toml")?;
/// }
/// ```
// fn parse<'a, T>(path: &str) -> Option<T> 
// where
//     T: Deserialize<'a>
// {
//     let path = PathBuf::from(path);
//     if !path.exists() {
//         return None;
//     }
//     let file = std::fs::read_to_string(path).ok()?;
//     let content = toml::from_slice::<T>(file.as_bytes()).ok()?; //TODO: Fix lifetime error.
//     Some(content)
// }

/// Gets the value from a struct by field name
/// 
/// Example:
/// ```rust
/// use libdmd::utils::config::writer::{get, Options};
/// fn main() {
///     let options = Options { name: "Devmode".to_string() };
///     let name: Option<String> = get::<Options, String>(&options, "name");
///     println!("{}", name.unwrap());
/// }
/// ```
pub fn get<T, R>(data: &T, field: &str) -> Option<R> 
where
    T: Serialize,
    R: DeserializeOwned
{
    let mut map = match serde_value::to_value(data) {
        Ok(Value::Map(map)) => map,
        _ => panic!("expected a struct")
    };
    let key = Value::String(field.to_owned());
    let value = match map.remove(&key) {
        Some(value) => value, 
        None => panic!("no such field")
    };

    match R::deserialize::<_>(value) {
        Ok(r) => Some(r),
        Err(_) => panic!("wrong type"),
    }
}

#[cfg(test)]
mod tests {
    use crate::utils::config::writer::*;
    #[test]
    fn check_get() {
        assert_eq!(get::<String>("name").unwrap(), "Eddy");
    }
}