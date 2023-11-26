#[cfg(test)]
mod tests {
    use crate::error::ParseError;
    use crate::helper::SerializeHelper;
    use crate::serializer::Serialize;
    use crate::values::Values;
    use crate::{map, r#box};

    #[derive(Debug)]
    pub struct S {
        a: String,
        b: String,
    }

    impl Serialize for S {
        fn serialize(self) -> Values {
            let first = Values::String(self.a);
            let second = Values::String(self.b);
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
            let first = Values::Number(self.a as f64);
            let second = Values::Object(r#box!(self.s.serialize()));
            Values::Struct(map!(("a".into(), first), ("s".into(), second)))
        }
    }

    impl Serialize for YourMum {
        fn serialize(self) -> Values {
            let mut vec = Vec::new();
            for items in self.0 {
                vec.push(items.serialize());
            }
            Values::Array(vec)
        }
    }

    #[derive(Debug)]
    struct YourMum(Vec<A>);

    #[test]
    fn test_one() {
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

        println!("Object: {:?}", &s);
        let map = s.serialize();
        println!("Abstraktion: {:?}", map);
        println!("JSON: {}", map);
        println!();
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
        let mum = YourMum(vec![
            A {
                a: 187,
                s: S {
                    a: "hello".to_string(),
                    b: "hello".to_string(),
                },
            },
            A {
                a: 19,
                s: S {
                    a: "hello".into(),
                    b: "hello".into(),
                },
            },
        ]);
        println!("{}", mum.serialize());



        let map_two = a.serialize();
        println!("Abstraktion: {:?}", map_two);
        println!("Json: {}", map_two);
        println!("Object: {:?}", A::try_from(&map_two).unwrap());
    }
}
