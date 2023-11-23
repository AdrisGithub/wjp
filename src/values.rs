use std::collections::HashMap;
use std::fmt::{Display, Formatter};

pub const STRING: &str = "string";
pub const OBJECT: &str = "object";
pub const STRUCT: &str = "struct";
pub const NUMBER: &str = "number";
pub const NONE: &str = "none";

#[derive(Debug, Clone)]
pub enum Values {
    String(String),
    Number(f64),
    Object(Box<Values>),
    Struct(HashMap<String, Values>),
    None,
}

impl Values {
    pub fn get_object(&self) -> Option<Values> {
        match self {
            Values::String(..) => None,
            Values::Number(..) => None,
            Values::Object(val) => Some(*val.clone()),
            Values::Struct(_) => None,
            Values::None => None,
        }
    }
    pub fn get_struct(&self) -> Option<HashMap<String, Values>> {
        match self {
            Values::String(..) => None,
            Values::Number(..) => None,
            Values::Object(..) => None,
            Values::Struct(map) => Some(map.clone()),
            Values::None => None,
        }
    }
    pub fn get_string(&self) -> Option<String> {
        match self {
            Values::String(string) => Some(string.to_string()),
            Values::Number(..) => None,
            Values::Object(..) => None,
            Values::Struct(_) => None,
            Values::None => None,
        }
    }
    pub fn get_number(&self) -> Option<f64> {
        match self {
            Values::String(..) => None,
            Values::Number(num) => Some(*num),
            Values::Object(..) => None,
            Values::Struct(_) => None,
            Values::None => None,
        }
    }
    pub fn get_type_as_string(&self) -> &str {
        match self {
            Values::String(_) => STRING,
            Values::Number(_) => NUMBER,
            Values::Object(_) => OBJECT,
            Values::Struct(_) => STRUCT,
            Values::None => NONE,
        }
    }
    pub fn is_none(&self) -> bool {
        self.get_type_as_string().eq(NONE)
    }
    pub fn is_string(&self) -> bool {
        self.get_type_as_string().eq(STRING)
    }
    pub fn is_object(&self) -> bool {
        self.get_type_as_string().eq(OBJECT)
    }
    pub fn is_number(&self) -> bool {
        self.get_type_as_string().eq(NUMBER)
    }
    pub fn is_struct(&self) -> bool {
        self.get_type_as_string().eq(STRUCT)
    }
}
