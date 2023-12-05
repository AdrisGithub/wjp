use std::collections::{BTreeMap, BTreeSet, HashMap, HashSet};
use std::isize;
use std::str::FromStr;

use crate::error::ParseError;
use crate::values::Values;

pub trait Serialize {
    fn serialize(&self) -> Values;
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

impl<K: ToString, V: Serialize> Serialize for HashMap<K, V> {
    fn serialize(&self) -> Values {
        let mut map = HashMap::with_capacity(4);
        for (k, v) in self.iter() {
            map.insert(k.to_string(), v.serialize());
        }
        Values::Struct(map)
    }
}

impl<I: Serialize> Serialize for HashSet<I> {
    fn serialize(&self) -> Values {
        Values::Array(self.iter().map(|val| val.serialize()).collect())
    }
}

impl<K: ToString, V: Serialize> Serialize for BTreeMap<K, V> {
    fn serialize(&self) -> Values {
        let mut map = HashMap::with_capacity(4);
        for (k, v) in self.iter() {
            map.insert(k.to_string(), v.serialize());
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
            .chars().next()
            .ok_or(ParseError::new())
    }
}

impl TryFrom<Values> for String {
    type Error = ParseError;
    fn try_from(value: Values) -> Result<Self, Self::Error> {
        value.get_string().ok_or(ParseError::new())
    }
}

impl TryFrom<Values> for usize {
    type Error = ParseError;
    fn try_from(value: Values) -> Result<Self, Self::Error> {
        value.get_number()
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
        value.get_bool()
            .ok_or(ParseError::new())
    }
}