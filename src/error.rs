use std::fmt::{Debug, Error, Formatter};

/// Error Struct that contains different Information's on what went wrong
#[derive(Eq, PartialOrd, PartialEq, Hash, Clone, Default, Ord)]
pub struct ParseError(String);

impl ParseError {
    /// constructs a new ParseError with an empty message
    pub const fn new() -> Self {
        Self(String::new())
    }
    /// replaces the Error Message with a provided Message
    pub fn with_msg(mut self, msg: &str) -> Self {
        self.0 = String::from(msg);
        self
    }
}

impl From<Error> for ParseError {
    fn from(value: Error) -> Self {
        ParseError::from(value.to_string())
    }
}

impl From<()> for ParseError {
    fn from(_value: ()) -> Self {
        Self::new()
    }
}
impl From<String> for ParseError {
    fn from(value: String) -> Self {
        ParseError(value)
    }
}

impl Debug for ParseError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
