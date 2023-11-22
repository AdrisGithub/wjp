#[macro_export]
macro_rules! map (
    {$(($key:expr,$value:expr)), + } => {
        {
            let mut m = std::collections::HashMap::new();
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