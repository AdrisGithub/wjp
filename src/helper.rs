use std::collections::HashMap;
use std::error::Error;

use crate::error::ParseError;
use crate::values::Values;

pub trait SerializeHelper<T> {
    fn get_val_unsafe(&self, attr: &str, fun: fn(&Values) -> Option<T>) -> T;
    fn get_val_opt(&self, attr: &str, fun: fn(&Values) -> Option<T>) -> Option<T>;
    fn get_val_res(&self, attr: &str, fun: fn(&Values) -> Option<T>) -> Result<T, ParseError>;
    fn rm_val(&mut self, attr: &str, fun: fn(Values) -> Option<T>) -> Result<T, ParseError>;
    fn map_val(
        &mut self,
        attr: &str,
        fun: fn(Values) -> Result<T, ParseError>,
    ) -> Result<T, ParseError>;
    fn map_ref_val(
        &mut self,
        attr: &str,
        fun: fn(&Values) -> Result<T, ParseError>,
    ) -> Result<T, ParseError>;
    fn map_val_and_err<E: Error>(
        &mut self,
        attr: &str,
        fun: fn(Values) -> Result<T, E>,
    ) -> Result<T, ParseError>;
    fn map_opt_val(
        &mut self,
        attr: &str,
        fun: fn(Values) -> Option<T>,
    ) -> Result<Option<T>, ParseError>;
}

impl<T> SerializeHelper<T> for HashMap<String, Values> {
    fn get_val_unsafe(&self, attr: &str, fun: fn(&Values) -> Option<T>) -> T {
        SerializeHelper::get_val_res(self, attr, fun).unwrap()
    }
    fn get_val_opt(&self, attr: &str, fun: fn(&Values) -> Option<T>) -> Option<T> {
        SerializeHelper::get_val_res(self, attr, fun).ok()
    }
    fn get_val_res(&self, attr: &str, fun: fn(&Values) -> Option<T>) -> Result<T, ParseError> {
        self.get(&String::from(attr))
            .map(fun)
            .ok_or(ParseError::new())?
            .ok_or(ParseError::new())
    }
    fn rm_val(&mut self, attr: &str, fun: fn(Values) -> Option<T>) -> Result<T, ParseError> {
        self.remove(&String::from(attr))
            .map(fun)
            .ok_or(ParseError::new())?
            .ok_or(ParseError::new())
    }
    fn map_val(
        &mut self,
        attr: &str,
        fun: fn(Values) -> Result<T, ParseError>,
    ) -> Result<T, ParseError> {
        self.remove(&String::from(attr))
            .map(fun)
            .ok_or(ParseError::new())?
    }
    fn map_ref_val(
        &mut self,
        attr: &str,
        fun: fn(&Values) -> Result<T, ParseError>,
    ) -> Result<T, ParseError> {
        self.get(&String::from(attr))
            .map(fun)
            .ok_or(ParseError::new())?
    }
    fn map_val_and_err<E: Error>(
        &mut self,
        attr: &str,
        fun: fn(Values) -> Result<T, E>,
    ) -> Result<T, ParseError> {
        self.remove(&String::from(attr))
            .map(fun)
            .ok_or(ParseError::new())?
            .map_err(|err| ParseError::new())
    }
    fn map_opt_val(
        &mut self,
        attr: &str,
        fun: fn(Values) -> Option<T>,
    ) -> Result<Option<T>, ParseError> {
        self.remove(attr).map(fun).ok_or(ParseError::new())
    }
}
