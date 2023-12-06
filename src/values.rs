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
            _ => false
        }
    }
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
                Values::Struct(map!(("hello",self.hello.serialize())))
            }
        }
        let struc = Hello { hello: String::from("Moin") }.serialize();
        assert_eq!(struc.to_string(), "{\"hello\":\"Moin\"}");
    }
    #[test]
    pub fn display_on_arr_with_struct() {
        struct Hello {
            hello: String,
        }
        impl Serialize for Hello {
            fn serialize(&self) -> Values {
                Values::Struct(map!(("hello",self.hello.serialize())))
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
                hello: String::from("Hello"),
            },
        ];
        assert_eq!(
            arr.serialize().to_string(),
            "[{\"hello\":\"Moin\"},{\"hello\":\"IDK\"},{\"hello\":\"Hello\"}]"
        );
    }
}