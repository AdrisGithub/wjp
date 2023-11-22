use crate::values::Values;

pub trait Serialize {
    fn serialize(self) -> Values;
}