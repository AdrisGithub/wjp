/// This macro can be used when implementing the [`Serialize`] Trait
/// Example:
/// ```rust
/// use std::collections::HashMap;
/// use wjp::map;
/// use wjp::Values;
///
/// let mut map = HashMap::new();
/// map.insert("test".to_string(),Values::Null);
///
/// assert_eq!(
///     map!(("test",Values::Null)),
///     map
/// )
///
/// ```
///
/// [`Serialize`]: crate::serializer::Serialize
#[macro_export]
macro_rules! map (
    () => {
      std::collections::HashMap::new()
    };
    ($(($key:expr,$value:expr)), + ) => {
        {
            let mut m = std::collections::HashMap::with_capacity(3);
            $(
                m.insert(String::from($key), $value);
             )+
            m
        }
    };
);

#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    #[test]
    pub fn with_empty_params() {
        assert_eq!(map!(), HashMap::<&str, &str>::new())
    }
    #[test]
    pub fn with_filled_params() {
        let mut map = HashMap::new();
        map.insert(String::from("test"), 123);
        assert_eq!(map!(("test", 123)), map)
    }
}
