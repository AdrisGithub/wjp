use std::fmt::{Debug, Formatter};

pub struct ParseError(());

impl ParseError {
    pub const fn new() -> Self {
        Self(())
    }
}

impl From<()> for ParseError {
    fn from(_value: ()) -> Self {
        ParseError(())
    }
}

impl Debug for ParseError {
    fn fmt(&self, _f: &mut Formatter<'_>) -> std::fmt::Result {
        Ok(())
    }
}