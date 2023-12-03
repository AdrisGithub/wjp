use std::collections::{BTreeMap, BTreeSet, HashMap, HashSet};

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
impl<T: Serialize> Serialize for &[T]{
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
