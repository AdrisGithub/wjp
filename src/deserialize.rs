use crate::error::ParseError;
use crate::parser::Parser;
use crate::values::Values;
/// Auto Trait for Deserializing JSON into predefined Structs.
///
/// The Methods in this trait will appear when implementing the [`TryFrom<Values>`] trait
/// and is used to deserialize JSON strings.
///
/// Usage:
/// ```
/// use wjp::{Deserialize, ParseError, SerializeHelper, Values};
/// // Example Struct to show how this library works
/// #[derive(Debug)]
/// struct Example {
///     code: f32,
///     messages: Vec<String>,
///     opt: Option<bool>,
/// }
/// // Implementing the TryFrom<Values> Trait allows you to deserialize a JSON String into your struct
/// impl TryFrom<Values> for Example {
///     // We advise on using the ParseError because many helper methods build on this error
///     type Error = ParseError;
///     fn try_from(value: Values) -> Result<Self, Self::Error> {
///         // Now you just need to get your struct / array and get the keys with their appropriate values
///         let mut struc = value.get_struct().ok_or(ParseError::new())?;
///         let code = struc.map_val("code", f32::try_from)?;
///         // Many Data Structures and Types already have TryFrom<Values> implemented
///         let messages = struc.map_val("messages", Vec::try_from)?;
///         // This is sadly not the case for Option<T> where your need to find out what the type of T is and parse that
///         let opt = struc.map_opt_val("opt", |val| val.get_bool())?;
///         Ok(Self {
///             opt,
///             messages,
///             code,
///         })
///     }
/// }
///
/// //And the <Your Type>::deserialize(&str/String) to deserialize it
/// let json = "{\"opt\":null,\"code\":123,\"messages\":[\"Important\",\"Message\"]}";
/// let back = Example::deserialize_str(json);
/// println!("{:?}", back);
/// ```
/// Info: The [`TryFrom<Values>`] impl needs to have Error = ParseError.
///
/// Implementing this Trait on any Struct won't bring any benefits
///
/// [`TryFrom<Values>`]: TryFrom
pub trait Deserialize: TryFrom<Values, Error = ParseError> {
    /// deserialize a [`String`] containing JSON into the provided Struct
    fn deserialize(str: String) -> Result<Self, ParseError> {
        Self::deserialize_str(str.as_str())
    }
    /// deserialize a &str containing JSON into the provided Struct
    fn deserialize_str(str: &str) -> Result<Self, ParseError> {
        Parser::new(str).parse().map(Self::try_from)?
    }
}

impl<T> Deserialize for T where T: TryFrom<Values, Error = ParseError> {}

#[cfg(test)]
mod tests {

    #[test]
    pub fn test() {}
}
