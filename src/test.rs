#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use crate::deserialize::Deserialize;
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
        fn serialize(&self) -> Values {
            Values::Struct(map!(("a", &self.a), ("b", &self.b)))
        }
    }

    impl TryFrom<Values> for S {
        type Error = ParseError;
        fn try_from(value: Values) -> Result<Self, Self::Error> {
            let s = value.get_struct().ok_or(())?;
            let a = s.get_val_res("a", |e| e.get_string())?;
            let b = s.get_val_res("b", |e| e.get_string())?;
            Ok(S { a, b })
        }
    }

    #[derive(Debug)]
    pub struct A {
        a: f64,
        s: S,
    }

    impl TryFrom<Values> for A {
        type Error = ParseError;
        fn try_from(value: Values) -> Result<Self, Self::Error> {
            let mut a = value.get_struct().ok_or(())?;
            let num = a.get_val_res("a", |v| v.get_number())?;
            let s = a.map_val("s", S::try_from)?;
            Ok(A { a: num, s })
        }
    }

    impl Serialize for A {
        fn serialize(&self) -> Values {
            Values::Struct(map!(("a", &self.a), ("s", &self.s)))
        }
    }

    impl Serialize for YourMum {
        fn serialize(&self) -> Values {
            let mut vec = Vec::new();
            for items in &self.0 {
                vec.push(items.serialize());
            }
            Values::Array(vec)
        }
    }

    #[derive(Debug)]
    struct YourMum(Vec<A>);

    impl TryFrom<Values> for YourMum {
        type Error = ParseError;
        fn try_from(value: Values) -> Result<Self, Self::Error> {
            Ok(Self(Vec::try_from(value)?))
        }
    }

    #[test]
    fn test_one() {
        let map = map!(("a", &1), ("a", &1));
        println!("{:?}", map.get("a"));
    }

    #[test]
    fn test() {
        let s = S {
            a: "Hey Joschka".into(),
            b: "ich habs".into(),
        };

        println!("Object: {:?}", &s);
        let map = s.serialize();
        println!("Abstraktion: {:?}", map);
        println!("JSON: {}", map);
        println!();
        let back_s = S::try_from(map).unwrap();
        println!("{:?}", back_s);
        println!("{}", back_s.a);

        let a = A {
            a: 187.0,
            s: S {
                a: "hello".to_string(),
                b: "hello".to_string(),
            },
        };

        let b = A { a: 299.0, s };

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
            A {
                a: 19.098888,
                s: S {
                    a: "l".to_string(),
                    b: "hello".into(),
                },
            },
            A {
                a: 19.098888,
                s: S {
                    a: "l".to_string(),
                    b: "hello".into(),
                },
            },
            A {
                a: 19.098888,
                s: S {
                    a: "l".to_string(),
                    b: "hello".into(),
                },
            },
            b,
        ]);
        println!("{}", mum.serialize());
        let map_two = a.serialize();
        println!("Abstraktion: {:?}", map_two);
        println!("Json: {}", map_two);
        println!("Object: {:?}", A::try_from(map_two).unwrap());

        println!("{:?}", f64::from_str("0.1999"));

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
        println!("{:?}", YourMum::deserialize(ser_mom));
        let contents = String::from("{\"ab\":\"c\\\"d\\\"e\"}");
        println!("{:?}", Parser::new(contents.as_str()).parse());
        let hello = String::from("Hello");
        println!("{}", hello.serialize());
    }
}
