# WJP - Wizards JSON Parser

A library to parse raw Strings into 
workable types and vice versa.

[![Latest version](https://img.shields.io/badge/crates.io-0.1.1-red)](https://crates.io/crates/wjp)
[![Documentation](https://docs.rs/log/badge.svg)](https://docs.rs/wjp)

## Documentation:

* [`wjp`](https://docs.rs/wjp)

## Basic Usage:

```rust

#[derive(Debug)]
struct Example {
    code: f32,
    messages: Vec<String>,
    opt: Option<bool>,
}

impl Serialize for Example {
    fn serialize(&self) -> Values {
        Values::Struct(map!(
            ("code", self.code.serialize()),
            ("messages", self.messages.serialize()),
            ("opt", self.opt.serialize())
        ))
    }
}
impl TryFrom<Values> for Example {
    type Error = ParseError;
    fn try_from(value: Values) -> Result<Self, Self::Error> {
        let mut struc = value.get_struct().ok_or(ParseError::new())?;
        let code = struc.map_val("code", f32::try_from)?;
        let messages = struc.map_val("messages", Vec::try_from)?;
        let opt = struc.map_opt_val("opt", |val| val.get_bool())?;
        Ok(Self {
            opt,
            messages,
            code,
        })
    }
}
pub fn main() {
    let example = Example {
        code: 123.0,
        messages: vec!["Important".to_string(), "Message".to_string()],
        opt: None,
    };
    let json = example.json();
    println!("{}", json);
    let back = Example::deserialize(json);
    println!("{:?}", back);
}

```

```text
{"messages":["Important","Message"],"code":123,"opt":null}

Ok(Example { code: 123.0, messages: ["Message", "Important"], opt: None })
```

## Explanation:

[JSON](https://datatracker.ietf.org/doc/html/rfc8259) 
is a lightweight, text-based, language-independent syntax for defining data 
interchange formats. Despite being language independent. It is not really optimised
for the [Rust](https://www.rust-lang.org/) language. 

##### Example Json:

```json
{
  "type": "error",
  "message": "A really bad Error occurred",
  "code": 444
}
```

---

##### Key Value Pairs can have different positions every time:

```json
{
  "code": 444,
  "message": "A really bad Error occurred",
  "type": "error"
}
```

This is currently also the case for this library because the Struct implementation uses a 
HashMap that allocates Key-Value Pairs every time at a new place and delivers them at different positions

---

##### Key Value pairs can just not exist, have the value null or be a different type:

```json
{
  "code": null,
  "type": 123.23
}
```

This is supported, but it makes the parsing part more difficult and is the Reason why 
the User of this library needs to implement `From<Values>` and `Serialize` for each of
their Structs they want to parse

---

##### JSON supports the [IEEE 754](https://de.wikipedia.org/wiki/IEEE_754) Standard for storing numbers:

```json
{
  "plus": 1,
  "minus": -1,
  "minus-zero": -0,
  "exponent": 10E10,
  "minus-exponent": 10e-10,
  "decimal-point": 1.2,
  "decimal-point-exponent": 1.23E-10
}
```
This is supported, but just uses the `f64::from_str()` underneath which should support all these cases 

---

##### JSON also supports the [UTF-8](https://datatracker.ietf.org/doc/html/rfc3629) Encoding

```json
{
  "text": "\u1234",
  "info": "  ^ This is not supported "
}
```
This library doesn't support \u escaped characters and doesn't advise on using escaped chars at all


---

##### JSON also supports different types inside of Arrays 

```json
[
  true,
  false,
  null,
  1.23,
  false
]
```
This is supported, but the User needs to find out what to do with
the `Vec<Values>` that can contain different types
