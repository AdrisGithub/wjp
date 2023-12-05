use crate::error::ParseError;
use crate::parser::Parser;
use crate::values::Values;

pub trait Deserialize: TryFrom<Values, Error=ParseError> {
    fn deserialize(str: String) -> Result<Self, ParseError> {
        Self::deserialize_str(str.as_str())
    }
    fn deserialize_str(str: &str) -> Result<Self, ParseError> {
        Parser::new(str)
            .parse()
            .map(Self::try_from)?
    }
}

impl<T> Deserialize for T where T: TryFrom<Values, Error=ParseError> {}


#[cfg(test)]
mod tests {

    #[test]
    pub fn test() {}
}