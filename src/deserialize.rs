use crate::error::ParseError;
use crate::parser::Parser;
use crate::values::Values;

pub trait Deserialize: TryFrom<Values, Error=ParseError> {
    fn deserialize(str: String) -> Result<Self, ParseError> {
        Parser::new(str.as_str())
            .parse()
            .map(|val| Self::try_from(val))?
    }
}

impl<T: TryFrom<Values, Error=ParseError>> Deserialize for T {}