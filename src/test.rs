#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use crate::error::ParseError;
    use crate::helper::SerializeHelper;
    use crate::map;
    use crate::parser::Parser;
    use crate::serializer::Serialize;
    use crate::values::Values;

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
        a: f64,
        s: S,
    }

    impl TryFrom<&Values> for A {
        type Error = ParseError;
        fn try_from(value: &Values) -> Result<Self, Self::Error> {
            let a = value.get_struct().ok_or(())?;
            let num = a.get_result("a".into(), |v| v.get_number())?;
            let object = a.get("s").ok_or(())?;
            let s = S::try_from(object)?;
            Ok(A { a: num, s })
        }
    }

    impl Serialize for A {
        fn serialize(self) -> Values {
            let first = Values::Number(self.a);
            Values::Struct(map!(("a".into(), first), ("s".into(), self.s.serialize())))
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

    impl TryFrom<&Values> for YourMum {
        type Error = ParseError;
        fn try_from(value: &Values) -> Result<Self, Self::Error> {
            let arr = value.get_list();
            let mut vec = Vec::new();
            for item in arr {
                vec.push(A::try_from(&item)?)
            }
            Ok(Self(vec))
        }
    }

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
            a: 187.0,
            s: S {
                a: "hello".to_string(),
                b: "hello".to_string(),
            },
        };

        let map_two = a.serialize();
        println!("Abstraktion: {:?}", map_two);
        println!("Json: {}", map_two);
        println!("Object: {:?}", A::try_from(&map_two).unwrap());

        println!("{:?}", f64::from_str("0.1999"));

        println!();
        println!();
        println!();

        let mum = YourMum(vec![
            A {
                a: 18.7,
                s: S {
                    a: "he\\\"llo".to_string(),
                    b: "hello".to_string(),
                },
            },
            A {
                a: 19.098888,
                s: S {
                    a: "l".to_string(),
                    b: "hello".into(),
                },
            },
        ]);
        let ser_mom = mum.serialize().to_string();
        println!("{}", ser_mom);
        let mut parser = Parser::new(ser_mom.as_str());
        println!("{:?}", parser.parse());
        let contents = String::from("{\"ab\":\"c\\\"d\\\"e\"}");
        println!("{:?}", Parser::new(contents.as_str()).parse());
    }
}
