use std::collections::HashMap;

use crate::error::ParseError;
use crate::values::Values;

pub trait SerializeHelper<T> {
    fn get_unchecked(&self, attr: String, fun: fn(&Values) -> Option<T>) -> T;
    fn get_optional(&self, attr: String, fun: fn(&Values) -> Option<T>) -> Option<T>;
    fn get_result(&self, attr: String, fun: fn(&Values) -> Option<T>) -> Result<T, ParseError>;
}

impl<T> SerializeHelper<T> for HashMap<String, Values> {
    fn get_unchecked(&self, attr: String, fun: fn(&Values) -> Option<T>) -> T {
        SerializeHelper::get_optional(self, attr, fun)
            .unwrap()
    }
    fn get_optional(&self, attr: String, fun: fn(&Values) -> Option<T>) -> Option<T> {
        SerializeHelper::get_result(self, attr, fun)
            .ok()
    }
    fn get_result(&self, attr: String, fun: fn(&Values) -> Option<T>) -> Result<T, ParseError> {
        self.get(&attr)
            .map(fun)
            .ok_or(ParseError::new())?
            .ok_or(ParseError::new())
    }
}