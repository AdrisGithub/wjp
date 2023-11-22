#[cfg(test)]
mod tests {
    use crate::error::ParseError;
    use crate::helper::SerializeHelper;
    use crate::map;
    use crate::serializer::Serialize;
    use crate::values::Values;

    #[derive(Debug)]
    pub struct S {
        a: String,
        b: String,
    }

    impl Serialize for S {
        fn serialize(self) -> Values {
            let first = Values::String("a".into(), self.a);
            let second = Values::String("b".into(), self.b);
            Values::Struct(map!(("a".into(), first), ("b".into(), second)))
        }
    }

    impl TryFrom<&Values> for S {
        type Error = ParseError;
        fn try_from(value: &Values) -> Result<Self, Self::Error> {
            let s = value.get_struct().ok_or(())?;
            let a = s.get_result("a".into(), |e| e.get_string())?;
            let b = s.get_result("b".into(), |e| e.get_string())?;
            Ok(S { a, b })
        }
    }

    #[derive(Debug)]
    pub struct A {
        a: u32,
        s: S,
    }

    impl TryFrom<&Values> for A {
        type Error = ParseError;
        fn try_from(value: &Values) -> Result<Self, Self::Error> {
            let a = value.get_struct().ok_or(())?;
            let num = a.get_result("a".into(), |v| v.get_number())?;
            let object = a.get("s").ok_or(())?.get_object().ok_or(())?;
            let s = S::try_from(&object)?;
            Ok(A { a: num as u32, s })
        }
    }

    impl Serialize for A {
        fn serialize(self) -> Values {
            let first = Values::Number("a".into(), self.a as f64);
            let second = Values::Object("s".into(), Box::new(self.s.serialize()));
            Values::Struct(map!(("a".into(), first), ("s".into(), second)))
        }
    }


    #[test]
    fn testone() {
        let map = map!(("a", 1), ("a", 1));
        println!("{:?}", map.get("a"));
    }

    #[test]
    fn test() {
        println!();
        println!();

        let s = S {
            a: "Hey Joschka".into(),
            b: "ich habs".into(),
        };
        let map = s.serialize();
        println!("{:?}", map);
        let back_s = S::try_from(&map).unwrap();
        println!("{:?}", back_s);
        println!("{}", back_s.a);

        let a = A {
            a: 187,
            s: S {
                a: "hello".to_string(),
                b: "hello".to_string(),
            },
        };
        let map_two = a.serialize();
        println!("{:?}", map_two);
        println!("{:?}", A::try_from(&map_two));
    }
}
