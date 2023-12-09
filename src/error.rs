use std::fmt::{Debug, Error, Formatter};

pub struct ParseError(String);

impl ParseError {
    pub const fn new() -> Self {
        Self(String::new())
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
