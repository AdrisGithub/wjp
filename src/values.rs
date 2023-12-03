use std::collections::HashMap;
use std::fmt::{Display, Formatter};

pub const STRING: &str = "string";
pub const STRUCT: &str = "struct";
pub const NUMBER: &str = "number";
pub const NULL: &str = "null";
pub const ARRAY: &str = "array";
pub const BOOLEAN: &str = "boolean";

#[derive(Debug, Clone)]
pub enum Values {
    String(String),
    Number(f64),
    Struct(HashMap<String, Values>),
    Array(Vec<Values>),
    Null,
    Boolean(bool),
}

impl Values {
    pub fn get_struct(&self) -> Option<HashMap<String, Values>> {
        match self {
            Values::Struct(map) => Some(map.clone()),
            _ => None,
        }
    }
    pub fn get_bool(&self) -> Option<bool> {
        match self {
            Values::Boolean(bool) => Some(*bool),
            _ => None,
        }
    }
    pub fn get_string(&self) -> Option<String> {
        match self {
            Values::String(string) => Some(string.to_string()),
            _ => None,
        }
    }
    pub fn get_number(&self) -> Option<f64> {
        match self {
            Values::Number(num) => Some(*num),
            _ => None,
        }
    }
    pub fn get_list_opt(&self) -> Option<Vec<Values>> {
        match self {
            Values::Array(arr) => Some(arr.to_vec()),
            _ => None,
        }
    }
    pub fn get_list(&self) -> Vec<Values> {
        self.get_list_opt().unwrap_or_default()
    }
    pub fn get_type_as_string(&self) -> &str {
        match self {
            Values::String(_) => STRING,
            Values::Number(_) => NUMBER,
            Values::Struct(_) => STRUCT,
            Values::Null => NULL,
            Values::Array(_) => ARRAY,
            Values::Boolean(_) => BOOLEAN,
        }
    }
    pub fn is_bool(&self) -> bool {
        self.get_type_as_string().eq(BOOLEAN)
    }
    pub fn is_null(&self) -> bool {
        self.get_type_as_string().eq(NULL)
    }
    pub fn is_string(&self) -> bool {
        self.get_type_as_string().eq(STRING)
    }
    pub fn is_number(&self) -> bool {
        self.get_type_as_string().eq(NUMBER)
    }
    pub fn is_struct(&self) -> bool {
        self.get_type_as_string().eq(STRUCT)
    }
    pub fn is_array(&self) -> bool {
        self.get_type_as_string().eq(ARRAY)
    }
}

impl Display for Values {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Values::String(string) => write!(f, "\"{}\"", string),
            Values::Number(number) => write!(f, "{}", number),
            Values::Struct(r#struct) => {
                write!(f, "{{")?;
                let mut first = true;
                for (key, val) in r#struct {
                    if first {
                        write!(f, "\"{}\":{}", key, val)?;
                        first = false;
                    } else {
                        write!(f, ",\"{}\":{}", key, val)?
                    }
                }
                write!(f, "}}")
            }
            Values::Array(arr) => {
                write!(f, "[")?;
                let mut first = true;
                for item in arr {
                    if first {
                        write!(f, "{}", item)?;
                        first = false;
                    } else {
                        write!(f, ",{}", item)?;
                    }
                }
                write!(f, "]")
            }
            Values::Null => write!(f, "{}", NULL),
            Values::Boolean(bool) => write!(f, "{}", bool),
        }
    }
}
