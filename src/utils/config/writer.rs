use std::panic;

use anyhow::Result;
use serde::{Deserialize, Serialize, de::{DeserializeOwned}};
use serde_value::Value;

pub trait Writer {
    fn write() -> Result<()>;
    fn set() -> Result<()>;
    /// Calls get_field_by_name().
    /// ```rust
    /// fn get<T>(&self, field: &str) -> Option<T> {
    ///     get_field_by_name(&self, "")
    /// }
    /// ```
    fn get<T: Writer + Serialize, R: DeserializeOwned>(&self, field: &str) -> Option<R>;
}

#[derive(Serialize, Deserialize)]
pub struct Default {
    name: String,
}

// impl Writer for Default {
//     fn write() -> Result<()> {
//         todo!()
//     }

//     fn set() -> Result<()> {
//         todo!()
//     }

//     fn get<T, R>(&self, field: &str) -> Option<R> 
//     where
//         T: Writer + Serialize, 
//         R: DeserializeOwned
//     {
//         let config = *self;
//         get_field_by_name::<T, R>(&config, field)
//     }
// }

/// Gets the value of a field by name.
/// ```rust
/// fn main() {
///     let x: Option<i32> = get_field_by_name::<Point, i32>(&Point { x: 2, y: 4 }, "x");
///     println!("{}", x.unwrap());
/// }
/// ```
pub fn get_field_by_name<T, R>(data: &T, field: &str) -> Option<R> 
where
    T: Writer + Serialize,
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