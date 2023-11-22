#[macro_export]
macro_rules! map (
    {$(($key:expr,$value:expr)), + } => {
        {
            use std::collections::HashMap;
            let mut m = HashMap::new();
            $(
                m.insert($key, $value);
             )+
            m
        }
    };
);
#[macro_export]
macro_rules! r#box {
    () => {};
    ($key:expr) => {
        Box::from($key)
    }
}