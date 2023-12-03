use std::collections::HashMap;

use crate::error::ParseError;
use crate::values::Values;

pub trait SerializeHelper<T> {
    fn get_unchecked(&self, attr: &str, fun: fn(&Values) -> Option<T>) -> T;
    fn get_optional(&self, attr: &str, fun: fn(&Values) -> Option<T>) -> Option<T>;
    fn get_result(&self, attr: &str, fun: fn(&Values) -> Option<T>) -> Result<T, ParseError>;
    fn rm_result(&mut self, attr: &str, fun: fn(Values) -> Option<T>) -> Result<T, ParseError>;
    fn parse_result(
        &mut self,
        attr: &str,
        fun: fn(Values) -> Result<T, ParseError>,
    ) -> Result<T, ParseError>;
    fn parse_ref_result(
        &mut self,
        attr: &str,
        fun: fn(&Values) -> Result<T, ParseError>,
    ) -> Result<T, ParseError>;
}

impl<T> SerializeHelper<T> for HashMap<String, Values> {
    fn get_unchecked(&self, attr: &str, fun: fn(&Values) -> Option<T>) -> T {
        SerializeHelper::get_result(self, attr, fun).unwrap()
    }
    fn get_optional(&self, attr: &str, fun: fn(&Values) -> Option<T>) -> Option<T> {
        SerializeHelper::get_result(self, attr, fun).ok()
    }
    fn get_result(&self, attr: &str, fun: fn(&Values) -> Option<T>) -> Result<T, ParseError> {
        self.get(&String::from(attr))
            .map(fun)
            .ok_or(ParseError::new())?
            .ok_or(ParseError::new())
    }
    fn rm_result(&mut self, attr: &str, fun: fn(Values) -> Option<T>) -> Result<T, ParseError> {
        self.remove(&String::from(attr))
            .map(fun)
            .ok_or(ParseError::new())?
            .ok_or(ParseError::new())
    }
    fn parse_result(
        &mut self,
        attr: &str,
        fun: fn(Values) -> Result<T, ParseError>,
    ) -> Result<T, ParseError> {
        self.remove(&String::from(attr))
            .map(fun)
            .ok_or(ParseError::new())?
    }
    fn parse_ref_result(
        &mut self,
        attr: &str,
        fun: fn(&Values) -> Result<T, ParseError>,
    ) -> Result<T, ParseError> {
        self.get(&String::from(attr))
            .map(fun)
            .ok_or(ParseError::new())?
    }
}
