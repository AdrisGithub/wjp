use std::collections::{BTreeMap, BTreeSet, HashMap, HashSet};
use std::hash::Hash;
use std::isize;
use std::str::FromStr;

use crate::deserialize::Deserialize;
use crate::error::ParseError;
use crate::values::Values;

/// Trait for Serializing Rust Structs into JSON
pub trait Serialize {
    /// This method is used to serialize your struct into a Values Object representing a JSON hierarchy
    ///
    /// Example:
    ///
    /// ```rust
    /// //Example Struct to show how this library works
    /// use wjp::{map, Serialize, Values};
    /// #[derive(Debug)]
    /// struct Example {
    ///     code: f32,
    ///     messages: Vec<String>,
    ///     opt: Option<bool>,
    /// }
    ///
    /// // Implementing the Serialize Trait allows you to call the .json() method on your struct
    /// impl Serialize for Example {
    ///     fn serialize(&self) -> Values {
    ///         // The map!() macro is a helper to create a hashmap from the given values
    ///         Values::Struct(map!(
    ///             // Many Data Structures and Types already have Serialize implemented
    ///             ("code", &self.code),
    ///             ("messages", &self.messages),
    ///             ("opt", &self.opt)
    ///         ))
    ///     }
    /// }
    /// let example = Example {
    ///     code: 123.0,
    ///     messages: vec!["Important".to_string(), "Message".to_string()],
    ///     opt: None,
    /// };   
    /// // After implementing these two traits you can call the .json() method to serialize your struct
    /// let json = example.json();
    /// println!("{}", json);
    /// ```
    fn serialize(&self) -> Values;
    /// This method has a default impl and it is not advised on writing your own impl for your structs
    fn json(&self) -> String {
        self.serialize().to_string()
    }
}

impl<S: Serialize> Serialize for Option<S> {
    fn serialize(&self) -> Values {
        match self {
            None => Values::Null,
            Some(s) => s.serialize(),
        }
    }
}

impl<R: Serialize, E: Serialize> Serialize for Result<R, E> {
    fn serialize(&self) -> Values {
        match self {
            Err(e) => e.serialize(),
            Ok(r) => r.serialize(),
        }
    }
}

impl<T: Serialize> Serialize for Vec<T> {
    fn serialize(&self) -> Values {
        Values::Array(self.iter().map(|e| e.serialize()).collect())
    }
}

impl<T: Serialize> Serialize for &[T] {
    fn serialize(&self) -> Values {
        Values::Array(self.iter().map(|e| e.serialize()).collect())
    }
}

impl<K: Serialize, V: Serialize> Serialize for HashMap<K, V> {
    fn serialize(&self) -> Values {
        let mut map = HashMap::with_capacity(4);
        for (k, v) in self.iter() {
            let mut string = k.serialize().to_string();
            string.remove(string.len()-1);
            string.remove(0);
            map.insert(string, v.serialize());
        }
        Values::Struct(map)
    }
}

impl<I: Serialize> Serialize for HashSet<I> {
    fn serialize(&self) -> Values {
        Values::Array(self.iter().map(|val| val.serialize()).collect())
    }
}

impl<K: Serialize, V: Serialize> Serialize for BTreeMap<K, V> {
    fn serialize(&self) -> Values {
        let mut map = HashMap::with_capacity(4);
        for (k, v) in self.iter() {
            let mut string = k.serialize().to_string();
            string.remove(string.len()-1);
            string.remove(0);
            map.insert(string, v.serialize());
        }
        Values::Struct(map)
    }
}

impl<I: Serialize> Serialize for BTreeSet<I> {
    fn serialize(&self) -> Values {
        Values::Array(self.iter().map(|val| val.serialize()).collect())
    }
}

impl Serialize for f32 {
    fn serialize(&self) -> Values {
        Values::Number(*self as f64)
    }
}

impl Serialize for f64 {
    fn serialize(&self) -> Values {
        Values::Number(*self)
    }
}

impl Serialize for str {
    fn serialize(&self) -> Values {
        Values::String(String::from(self))
    }
}

impl Serialize for &str {
    fn serialize(&self) -> Values {
        Serialize::serialize(*self)
    }
}

impl Serialize for String {
    fn serialize(&self) -> Values {
        Serialize::serialize(self.as_str())
    }
}

impl Serialize for char {
    fn serialize(&self) -> Values {
        Serialize::serialize(&self.to_string())
    }
}

impl Serialize for bool {
    fn serialize(&self) -> Values {
        Values::Boolean(*self)
    }
}

impl Serialize for usize {
    fn serialize(&self) -> Values {
        Values::Number(*self as f64)
    }
}

impl Serialize for u8 {
    fn serialize(&self) -> Values {
        Values::Number(*self as f64)
    }
}

impl Serialize for u16 {
    fn serialize(&self) -> Values {
        Values::Number(*self as f64)
    }
}

impl Serialize for u32 {
    fn serialize(&self) -> Values {
        Values::Number(*self as f64)
    }
}

impl Serialize for u64 {
    fn serialize(&self) -> Values {
        Values::Number(*self as f64)
    }
}

impl Serialize for u128 {
    fn serialize(&self) -> Values {
        Values::Number(*self as f64)
    }
}

impl Serialize for isize {
    fn serialize(&self) -> Values {
        Values::Number(*self as f64)
    }
}

impl Serialize for i8 {
    fn serialize(&self) -> Values {
        Values::Number(*self as f64)
    }
}

impl Serialize for i16 {
    fn serialize(&self) -> Values {
        Values::Number(*self as f64)
    }
}

impl Serialize for i32 {
    fn serialize(&self) -> Values {
        Values::Number(*self as f64)
    }
}

impl Serialize for i64 {
    fn serialize(&self) -> Values {
        Values::Number(*self as f64)
    }
}

impl Serialize for i128 {
    fn serialize(&self) -> Values {
        Values::Number(*self as f64)
    }
}

impl<T: TryFrom<Values>> TryFrom<Values> for Vec<T> {
    type Error = ParseError;
    fn try_from(value: Values) -> Result<Self, Self::Error> {
        let mut pre = value.get_list_opt().ok_or(ParseError::new())?;
        let mut post = Vec::with_capacity(pre.len());
        while !pre.is_empty() {
            post.push(T::try_from(pre.pop().unwrap()).map_err(|_err| ParseError::new())?)
        }
        Ok(post)
    }
}

impl TryFrom<Values> for char {
    type Error = ParseError;
    fn try_from(value: Values) -> Result<Self, Self::Error> {
        String::try_from(value)?
            .chars()
            .next()
            .ok_or(ParseError::new())
    }
}

impl TryFrom<Values> for String {
    type Error = ParseError;
    fn try_from(value: Values) -> Result<Self, Self::Error> {
        value.get_string().ok_or(ParseError::new())
    }
}

impl TryFrom<Values> for f32 {
    type Error = ParseError;
    fn try_from(value: Values) -> Result<Self, Self::Error> {
        f64::try_from(value).map(|val| val as f32)
    }
}

impl TryFrom<Values> for f64 {
    type Error = ParseError;
    fn try_from(value: Values) -> Result<Self, Self::Error> {
        value.get_number().ok_or(ParseError::new())
    }
}

impl TryFrom<Values> for usize {
    type Error = ParseError;
    fn try_from(value: Values) -> Result<Self, Self::Error> {
        value
            .get_number()
            .map(|f| f.to_string())
            .map(|s| usize::from_str(s.as_str()))
            .ok_or(ParseError::new())?
            .map_err(|_err| ParseError::new())
    }
}

impl TryFrom<Values> for u8 {
    type Error = ParseError;
    fn try_from(value: Values) -> Result<Self, Self::Error> {
        usize::try_from(value)
            .map(u8::try_from)?
            .map_err(|_err| ParseError::new())
    }
}

impl TryFrom<Values> for u16 {
    type Error = ParseError;
    fn try_from(value: Values) -> Result<Self, Self::Error> {
        usize::try_from(value)
            .map(u16::try_from)?
            .map_err(|_err| ParseError::new())
    }
}

impl TryFrom<Values> for u32 {
    type Error = ParseError;
    fn try_from(value: Values) -> Result<Self, Self::Error> {
        usize::try_from(value)
            .map(u32::try_from)?
            .map_err(|_err| ParseError::new())
    }
}

impl TryFrom<Values> for u64 {
    type Error = ParseError;
    fn try_from(value: Values) -> Result<Self, Self::Error> {
        usize::try_from(value)
            .map(u64::try_from)?
            .map_err(|_err| ParseError::new())
    }
}

impl TryFrom<Values> for u128 {
    type Error = ParseError;
    fn try_from(value: Values) -> Result<Self, Self::Error> {
        usize::try_from(value)
            .map(u128::try_from)?
            .map_err(|_err| ParseError::new())
    }
}

impl TryFrom<Values> for isize {
    type Error = ParseError;
    fn try_from(value: Values) -> Result<Self, Self::Error> {
        String::try_from(value)
            .map(|str| isize::from_str(str.as_str()))?
            .map_err(|_err| ParseError::new())
    }
}

impl TryFrom<Values> for i8 {
    type Error = ParseError;
    fn try_from(value: Values) -> Result<Self, Self::Error> {
        isize::try_from(value)
            .map(i8::try_from)?
            .map_err(|_err| ParseError::new())
    }
}

impl TryFrom<Values> for i16 {
    type Error = ParseError;
    fn try_from(value: Values) -> Result<Self, Self::Error> {
        isize::try_from(value)
            .map(i16::try_from)?
            .map_err(|_err| ParseError::new())
    }
}

impl TryFrom<Values> for i32 {
    type Error = ParseError;
    fn try_from(value: Values) -> Result<Self, Self::Error> {
        isize::try_from(value)
            .map(i32::try_from)?
            .map_err(|_err| ParseError::new())
    }
}

impl TryFrom<Values> for i64 {
    type Error = ParseError;
    fn try_from(value: Values) -> Result<Self, Self::Error> {
        isize::try_from(value)
            .map(i64::try_from)?
            .map_err(|_err| ParseError::new())
    }
}

impl TryFrom<Values> for i128 {
    type Error = ParseError;
    fn try_from(value: Values) -> Result<Self, Self::Error> {
        isize::try_from(value)
            .map(i128::try_from)?
            .map_err(|_err| ParseError::new())
    }
}

impl TryFrom<Values> for bool {
    type Error = ParseError;
    fn try_from(value: Values) -> Result<Self, Self::Error> {
        value.get_bool().ok_or(ParseError::new())
    }
}

impl<K, V> TryFrom<Values> for HashMap<K, V>
    where
        K: TryFrom<Values, Error=ParseError> + Eq + Hash,
        V: TryFrom<Values, Error=ParseError>,
{
    type Error = ParseError;
    fn try_from(value: Values) -> Result<Self, Self::Error> {
        let mut map = HashMap::new();
        for (key, value) in value.get_struct().ok_or(ParseError::new())? {
            map.insert(K::deserialize_str(key.as_str())?, V::try_from(value)?);
        }
        Ok(map)
    }
}

impl<K, V> TryFrom<Values> for BTreeMap<K, V>
    where
        K: TryFrom<Values, Error=ParseError> + Eq + Hash + Ord,
        V: TryFrom<Values, Error=ParseError>,
{
    type Error = ParseError;
    fn try_from(value: Values) -> Result<Self, Self::Error> {
        let mut map = BTreeMap::new();
        for (key, value) in value.get_struct().ok_or(ParseError::new())? {
            map.insert(Deserialize::deserialize(key)?, V::try_from(value)?);
        }
        Ok(map)
    }
}

impl<V> TryFrom<Values> for BTreeSet<V>
    where
        V: TryFrom<Values, Error=ParseError> + Ord,
{
    type Error = ParseError;
    fn try_from(value: Values) -> Result<Self, Self::Error> {
        let val = value.get_list_opt().ok_or(ParseError::new())?;
        let mut set = BTreeSet::new();
        for item in val {
            set.insert(V::try_from(item)?);
        }
        Ok(set)
    }
}

impl<V> TryFrom<Values> for HashSet<V>
    where
        V: TryFrom<Values, Error=ParseError> + Hash + Eq,
{
    type Error = ParseError;
    fn try_from(value: Values) -> Result<Self, Self::Error> {
        let val = value.get_list_opt().ok_or(ParseError::new())?;
        let mut set = HashSet::new();
        for item in val {
            set.insert(V::try_from(item)?);
        }
        Ok(set)
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use std::fmt::Display;

    use crate::{Deserialize, map, ParseError, SerializeHelper, Values};
    use crate::serializer::Serialize;

    #[test]
    pub fn test_serialized_option_none() {
        let none: Option<bool> = None;
        assert_eq!("null", none.serialize().to_string());
    }

    #[test]
    pub fn test_serialized_option_some() {
        let some = Some(true);
        assert_eq!("true", some.serialize().to_string());
    }

    #[test]
    pub fn test_serialized_result_err() {
        let string: Result<&str, &str> = Err("Hello I am a Error");
        assert_eq!("\"Hello I am a Error\"", string.serialize().to_string())
    }

    #[test]
    pub fn test_serialized_result_ok() {
        let num: Result<f64, &str> = Ok(123.22);
        assert_eq!("123.22", num.serialize().to_string())
    }

    #[test]
    pub fn test_serialized_vec_empty() {
        let arr: Vec<bool> = vec![];
        assert_eq!("[]", arr.serialize().to_string())
    }

    #[test]
    pub fn test_serialized_vec_filled() {
        let arr = vec![true, false, false, false];
        assert_eq!("[true,false,false,false]", arr.serialize().to_string())
    }

    #[test]
    pub fn test_serialized_map_empty() {
        let map: HashMap<String, String> = map!();
        assert_eq!("{}", map.serialize().to_string())
    }

    #[test]
    pub fn test_serialized_map_filled() {
        let map = map!(("Hello", &true));
        assert_eq!("{\"Hello\":true}", map.serialize().to_string())
    }

    #[test]
    pub fn test_serialized_map_filled_s() {
        #[derive(Hash, Eq, PartialEq, Debug)]
        struct IDK {
            map: u128,
        }
        impl Display for IDK {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, "{}", self.json())
            }
        }
        impl TryFrom<Values> for IDK {
            type Error = ParseError;
            fn try_from(value: Values) -> Result<Self, Self::Error> {
                let mut struc = value.get_struct().ok_or(ParseError::new())?;
                Ok(Self {
                    map: struc.map_val("map", u128::try_from)?
                })
            }
        }
        impl Serialize for IDK {
            fn serialize(&self) -> Values {
                Values::Struct(map!(("map",&self.map)))
            }
        }

        let mut map = HashMap::new();
        map.insert(100u8, IDK { map: 1 });
        let ser = map.json();
        println!("{}", ser);
        let back = HashMap::<u8, IDK>::deserialize(ser);
        println!("{:?}", back);
    }
}
