use std::collections::HashMap;
use std::fmt::{Display, Formatter};

/// Different Enums to construct an abstract JSON Hierarchy which is easier to work with and to construct
#[derive(Debug, Clone)]
pub enum Values {
    /// Represents a JSON String
    /// ```
    /// use wjp::Values;
    /// assert_eq!(
    ///     "\"Hello\"",
    ///     Values::String(String::from("Hello")).to_string()
    /// )
    /// ```
    String(String),
    /// Represents a JSON Number
    /// ```
    /// use wjp::Values;
    /// assert_eq!(
    ///     "12.43",
    ///     Values::Number(12.43).to_string()
    /// )
    /// ```
    Number(f64),
    /// Represents a JSON Struct
    /// ```
    /// use wjp::{map, Values};
    /// assert_eq!(
    ///     Values::Struct(map!(("message",Values::Null))).to_string(),
    ///     "{\"message\":null}"
    /// )
    /// ```
    Struct(HashMap<String, Values>),
    /// Represents a JSON Array
    /// ```
    /// use wjp::Values;
    /// assert_eq!(
    ///     Values::Array(vec![Values::Null,Values::Boolean(true)]).to_string(),
    ///     "[null,true]"
    /// )
    /// ```
    Array(Vec<Values>),
    /// Represents the JSON Value "null"
    /// ```
    /// use wjp::Values;
    /// assert_eq!(
    ///     Values::Null.to_string(),
    ///     "null"
    /// );
    /// ```
    Null,
    /// Represents the JSON Value "true" or "false"
    /// ```
    /// use wjp::Values;
    /// assert_eq!(
    ///     Values::Boolean(true).to_string(),
    ///     "true"  
    /// );
    /// assert_eq!(
    ///     Values::Boolean(false).to_string(),
    ///     "false"  
    /// )
    /// ```
    Boolean(bool),
}

impl PartialEq<Self> for Values {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (&Values::Null, &Values::Null) => true,
            (Values::String(a), Values::String(b)) => a == b,
            (&Values::Number(a), &Values::String(ref b))
            | (&Values::String(ref b), &Values::Number(a)) => a.to_string() == *b,
            (Values::Number(a), Values::Number(b)) => a == b,
            (Values::Boolean(a), Values::Boolean(b)) => a == b,
            (Values::Struct(a), Values::Struct(b)) => a == b,
            (Values::Array(a), Values::Array(b)) => a == b,
            _ => false,
        }
    }
}

impl Values {
    pub const STRING: &'static str = "string";
    pub const STRUCT: &'static str = "struct";
    pub const NUMBER: &'static str = "number";
    pub const NULL: &'static str = "null";
    pub const ARRAY: &'static str = "array";
    pub const BOOLEAN: &'static str = "boolean";
    /// if the provided value is a [`Struct`] it will return [`Some`]
    /// containing the inner [`Hashmap`] otherwise returns [`None`]
    ///
    /// [`Struct`]: Values::Struct
    /// [`Hashmap`]: HashMap
    pub fn get_struct(&self) -> Option<HashMap<String, Values>> {
        match self {
            Values::Struct(map) => Some(map.clone()),
            _ => None,
        }
    }
    /// if the provided value is a [`Boolean`] it will return [`Some`]
    /// containing the inner [`bool`] otherwise returns [`None`]
    ///
    /// [`Boolean`]: Values::Boolean
    pub fn get_bool(&self) -> Option<bool> {
        match self {
            Values::Boolean(bool) => Some(*bool),
            _ => None,
        }
    }
    /// if the provided value is a [`String`] it will return [`Some`]
    /// containing the inner [`str`] otherwise returns [`None`]
    ///
    /// [`String`]: Values::String
    /// [`str`]: String
    pub fn get_string(&self) -> Option<String> {
        match self {
            Values::String(string) => Some(string.to_string()),
            _ => None,
        }
    }
    /// if the provided value is a [`Number`] it will return [`Some`]
    /// containing the inner [`f64`] otherwise returns [`None`]
    ///
    /// [`Number`]: Values::Number
    pub fn get_number(&self) -> Option<f64> {
        match self {
            Values::Number(num) => Some(*num),
            _ => None,
        }
    }
    /// if the provided value is a [`Array`] it will return [`Some`]
    /// containing the inner [`Vec<Values>`] otherwise returns [`None`]
    ///
    /// [`Array`]: Values::Array
    pub fn get_list_opt(&self) -> Option<Vec<Values>> {
        match self {
            Values::Array(arr) => Some(arr.to_vec()),
            _ => None,
        }
    }
    /// if the provided value is a [`Array`] it will return it
    /// otherwise an empty Vec
    ///
    /// [`Array`]: Values::Array
    pub fn get_list(&self) -> Vec<Values> {
        self.get_list_opt().unwrap_or_default()
    }
    /// get the Type of this [`Values`] Object as a String
    /// It could be:
    ///     [`STRING`], [`NUMBER`], [`STRUCT`], [`NULL`], [`ARRAY`] or [`BOOLEAN`]
    ///
    ///[`STRING`]: Self::STRING
    ///[`NUMBER`]: Self::NUMBER
    ///[`STRUCT`]: Self::STRUCT
    ///[`NULL`]: Self::NULL
    ///[`ARRAY`]: Self::ARRAY
    ///[`BOOLEAN`]: Self::BOOLEAN
    pub fn get_type_as_string(&self) -> &str {
        match self {
            Values::String(_) => Self::STRING,
            Values::Number(_) => Self::NUMBER,
            Values::Struct(_) => Self::STRUCT,
            Values::Null => Self::NULL,
            Values::Array(_) => Self::ARRAY,
            Values::Boolean(_) => Self::BOOLEAN,
        }
    }
    /// returns true if the provided Value is [`Boolean`]
    ///
    /// [`Boolean`]: Values::Boolean
    pub fn is_bool(&self) -> bool {
        self.get_type_as_string().eq(Self::BOOLEAN)
    }
    /// returns true if the provided Value is [`Null`]
    ///
    /// [`Null`]: Values::Null
    pub fn is_null(&self) -> bool {
        self.get_type_as_string().eq(Self::NULL)
    }
    /// returns true if the provided Value is [`String`]
    ///
    /// [`String`]: Values::String
    pub fn is_string(&self) -> bool {
        self.get_type_as_string().eq(Self::STRING)
    }
    /// returns true if the provided Value is [`Number`]
    ///
    /// [`Number`]: Values::Number
    pub fn is_number(&self) -> bool {
        self.get_type_as_string().eq(Self::NUMBER)
    }
    /// returns true if the provided Value is [`Struct`]
    ///
    /// [`Struct`]: Values::Struct
    pub fn is_struct(&self) -> bool {
        self.get_type_as_string().eq(Self::STRUCT)
    }
    /// returns true if the provided Value is [`Array`]
    ///
    /// [`Array`]: Values::Array
    pub fn is_array(&self) -> bool {
        self.get_type_as_string().eq(Self::ARRAY)
    }
}

impl Display for Values {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Values::String(string) => write!(
                f,
                "\"{}\"",
                string.replace('\\', "\\\\").replace('\"', "\\\"")
            ),
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
            Values::Null => write!(f, "{}", Self::NULL),
            Values::Boolean(bool) => write!(f, "{}", bool),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::map;
    use crate::serializer::Serialize;
    use crate::values::Values;

    #[test]
    pub fn display_on_bool_true() {
        let bool = Values::Boolean(true);
        assert_eq!(bool.to_string(), "true")
    }

    #[test]
    pub fn display_on_bool_false() {
        let bool = Values::Boolean(false);
        assert_eq!(bool.to_string(), "false")
    }

    #[test]
    pub fn display_on_null() {
        let null = Values::Null;
        assert_eq!(null.to_string(), "null")
    }

    #[test]
    pub fn display_on_arr() {
        let arr = vec![Values::Null, Values::Null];
        let null = Values::Array(arr);
        assert_eq!(null.to_string(), "[null,null]")
    }

    #[test]
    pub fn display_on_num() {
        let num = Values::Number(1.56);
        assert_eq!(num.to_string(), "1.56")
    }

    #[test]
    pub fn display_on_string() {
        let num = Values::String(String::from("TEST TEST TEST"));
        assert_eq!(num.to_string(), "\"TEST TEST TEST\"")
    }

    #[test]
    pub fn display_on_struct() {
        struct Hello {
            hello: String,
        }
        impl Serialize for Hello {
            fn serialize(&self) -> Values {
                Values::Struct(map!(("hello", self.hello.serialize())))
            }
        }
        let struc = Hello {
            hello: String::from("Moin"),
        }
        .serialize();
        assert_eq!(struc.to_string(), "{\"hello\":\"Moin\"}");
    }
    #[test]
    pub fn display_on_arr_with_struct() {
        struct Hello {
            hello: String,
        }
        impl Serialize for Hello {
            fn serialize(&self) -> Values {
                Values::Struct(map!(("hello", self.hello.serialize())))
            }
        }
        let arr = vec![
            Hello {
                hello: String::from("Moin"),
            },
            Hello {
                hello: String::from("IDK"),
            },
            Hello {
                hello: String::from("Hel\"lo"),
            },
        ];
        assert_eq!(
            arr.serialize().to_string(),
            "[{\"hello\":\"Moin\"},{\"hello\":\"IDK\"},{\"hello\":\"Hel\\\"lo\"}]"
        );
    }
}
