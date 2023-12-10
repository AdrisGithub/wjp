use std::collections::HashMap;
use std::error::Error;

use crate::error::ParseError;
use crate::values::Values;
/// Helper Trait for Serializing JSON
pub trait SerializeHelper<T> {
    /// directly get T without any further checks
    /// Warning: this operation calls the [`.unwrap()`] method
    ///
    /// [`.unwrap()`]: Option::unwrap
    fn get_val_unsafe(&self, attr: &str, fun: fn(&Values) -> Option<T>) -> T;
    /// get an [`Option<T>`] without the Error message why the operation maybe failed
    ///
    /// [`Option<T>`]: Option
    fn get_val_opt(&self, attr: &str, fun: fn(&Values) -> Option<T>) -> Option<T>;
    /// get a Result of T or [`ParseError`] containing Info why the operation failed.
    /// In this case the function only takes a referenced [`Values`] object and returns an [`Option<T>`]
    ///
    /// [`Option<T>`]: Option
    fn get_val_res(&self, attr: &str, fun: fn(&Values) -> Option<T>) -> Result<T, ParseError>;
    /// get a Result of T or [`ParseError`] containing Info why the operation failed.
    /// In this case the function only takes a [`Values`] object and returns an [`Option<T>`]
    ///
    /// [`Option<T>`]: Option
    fn rm_val(&mut self, attr: &str, fun: fn(Values) -> Option<T>) -> Result<T, ParseError>;
    /// get a Result of T or [`ParseError`] containing Info why the operation failed.
    /// In this case the function only takes a [`Values`] object and returns an [`Result<T,ParseError>`]
    ///
    /// [`Result<T,ParseError>`]: Result
    fn map_val(
        &mut self,
        attr: &str,
        fun: fn(Values) -> Result<T, ParseError>,
    ) -> Result<T, ParseError>;
    /// get a Result of T or [`ParseError`] containing Info why the operation failed.
    /// In this case the function only takes a referenced [`Values`] object and returns an [`Result<T,ParseError>`]
    ///
    /// [`Result<T,ParseError>`]: Result
    fn map_ref_val(
        &mut self,
        attr: &str,
        fun: fn(&Values) -> Result<T, ParseError>,
    ) -> Result<T, ParseError>;
    /// get a Result of T or [`ParseError`] containing Info why the operation failed.
    /// In this case the function only takes a referenced [`Values`] object and returns an [`Result<T,E>`]
    ///
    /// [`Result<T,E>`]: Result
    fn map_val_and_err<E: Error>(
        &mut self,
        attr: &str,
        fun: fn(Values) -> Result<T, E>,
    ) -> Result<T, ParseError>;
    /// get a Result of [`Option<T>`] or [`ParseError`] containing Info why the operation failed.
    /// In this case the function only takes a referenced [`Values`] object and returns an [`Result<T,ParseError>`]
    ///
    /// [`Result<T,ParseError>`]: Result
    /// [`Option<T>`]: Option
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
            .map_err(|_err| ParseError::new())
    }
    fn map_opt_val(
        &mut self,
        attr: &str,
        fun: fn(Values) -> Option<T>,
    ) -> Result<Option<T>, ParseError> {
        self.remove(attr).map(fun).ok_or(ParseError::new())
    }
}
