//! ```
//! use wjp::{Deserialize, map, ParseError, Serialize, SerializeHelper, Values};
//!
//! // Example Struct to show how this library works
//! #[derive(Debug)]
//! struct Example {
//!     code: f32,
//!     messages: Vec<String>,
//!     opt: Option<bool>,
//! }
//!
//! // Implementing the Serialize Trait allows you to call the .json() method on your struct
//! impl Serialize for Example {
//!     fn serialize(&self) -> Values {
//!         // The map!() macro is a helper to create a hashmap from the given values
//!         Values::Struct(map!(
//!             // Many Data Structures and Types already have Serialize implemented
//!             ("code", &self.code),
//!             ("messages", &self.messages),
//!             ("opt", &self.opt)
//!         ))
//!     }
//! }
//!
//! // Implementing the TryFrom<Values> Trait allows you to deserialize a JSON String into your struct
//! impl TryFrom<Values> for Example {
//!     // We advise on using the ParseError because many helper methods build on this error
//!     type Error = ParseError;
//!     fn try_from(value: Values) -> Result<Self, Self::Error> {
//!         // Now you just need to get your struct / array and get the keys with their appropriate values
//!         let mut struc = value.get_struct().ok_or(ParseError::new())?;
//!         let code = struc.map_val("code", f32::try_from)?;
//!         // Many Data Structures and Types already have TryFrom<Values> implemented
//!         let messages = struc.map_val("messages", Vec::try_from)?;
//!         // This is sadly not the case for Option<T> where your need to find out what the type of T is and parse that
//!         let opt = struc.map_opt_val("opt", |val| val.get_bool())?;
//!         Ok(Self {
//!             opt,
//!             messages,
//!             code,
//!         })
//!     }
//! }
//! let example = Example { ///
//!     code: 123.0,
//!     messages: vec!["Important".to_string(), "Message".to_string()],
//!     opt: None,
//! };
//! // After implementing these two traits you can call the .json() method to serialize your struct
//! let json = example.json();
//! println!("{}", json);
//! // And the <Your Type>::deserialize(&str/String) to deserialize it
//! let back = Example::deserialize(json);
//! println!("{:?}", back);
//! ```
//! Output of the Example above:
//!
//! ```text
//! {"opt":null,"code":123,"messages":["Important","Message"]}
//!
//! Ok(Example { code: 123.0, messages: ["Important", "Message"], opt: None })
//! ```
//!
//!
pub use deserialize::Deserialize;
pub use error::ParseError;
pub use helper::SerializeHelper;
pub use serializer::Serialize;
pub use values::Values;
pub const NULL: Values = Values::Null;
pub const TRUE: Values = Values::Boolean(true);
pub const FALSE: Values = Values::Boolean(false);
mod deserialize;
mod error;
mod helper;
mod macros;
mod parser;
mod serializer;
#[cfg(test)]
mod test;
mod values;
