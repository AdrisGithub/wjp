use crate::error::ParseError;
use crate::parser::Parser;
use crate::values::Values;
/// Auto Trait for Deserializing JSON into predefined Structs.
///
/// The Methods in this trait will appear when implementing the [`TryFrom<Values>`] trait
/// and is used to deserialize JSON strings.
///
/// Usage:
/// ```rust;
/// use wjp::{Deserialize, ParseError, Values};
///
/// impl TryFrom<Values> for bool {
///     type Error = ParseError;
///     fn try_from(value: Values) -> Result<Self, Self::Error> {
///         value.get_bool().ok_or(ParseError::new())
///     }
/// }
///
/// let boolean = "true";
/// let parsed = bool::deserialize_str(boolean).unwrap();
/// assert_eq!(parsed,true);
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
