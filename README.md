# WJP - Wizards JSON Parser

A library to parse raw Strings into 
workable types and vice versa.

[![Latest version](https://img.shields.io/badge/crates.io-1.1.2-red)](https://crates.io/crates/wjp)
[![Documentation](https://docs.rs/log/badge.svg)](https://docs.rs/wjp)
[![Reliability Rating](https://sonarcloud.io/api/project_badges/measure?project=AdrisGithub_wjp&metric=reliability_rating)](https://sonarcloud.io/summary/new_code?id=AdrisGithub_wjp)
[![Quality Gate Status](https://sonarcloud.io/api/project_badges/measure?project=AdrisGithub_wjp&metric=alert_status)](https://sonarcloud.io/summary/new_code?id=AdrisGithub_wjp)
[![Technical Debt](https://sonarcloud.io/api/project_badges/measure?project=AdrisGithub_wjp&metric=sqale_index)](https://sonarcloud.io/summary/new_code?id=AdrisGithub_wjp)
## Documentation:

* [`wjp`](https://docs.rs/wjp)

## Basic Usage:

Import the library into your Cargo.toml

```toml
[dependencies]
wjp = "1.1.2"
```

```rust

// Example Struct to show how this library works
#[derive(Debug)]
struct Example {
    code: f32,
    messages: Vec<String>,
    opt: Option<bool>,
}

// Implementing the Serialize Trait allows you to call the .json() method on your struct
impl Serialize for Example {
    fn serialize(&self) -> Values {
        // The map!() macro is a helper to create a hashmap from the given values
        Values::Struct(map!(
            // Many Data Structures and Types already have Serialize implemented
            ("code", self.code.serialize()),
            ("messages", self.messages.serialize()),
            ("opt", self.opt.serialize())
        ))
    }
}

// Implementing the TryFrom<Values> Trait allows you to deserialize a JSON String into your struct
impl TryFrom<Values> for Example {
    // We advise on using the ParseError because many helper methods build on this error
    type Error = ParseError;
    fn try_from(value: Values) -> Result<Self, Self::Error> {
        // Now you just need to get your struct / array and get the keys with their appropriate values
        let mut struc = value.get_struct().ok_or(ParseError::new())?;
        let code = struc.map_val("code", f32::try_from)?;
        // Many Data Structures and Types already have TryFrom<Values> implemented
        let messages = struc.map_val("messages", Vec::try_from)?;
        // This is sadly not the case for Option<T> where your need to find out what the type of T is and parse that
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
    // After implementing these two traits you can call the .json() method to serialize your struct
    let json = example.json();
    println!("{}", json);
    // And the <Your Type>::deserialize(&str/String) to deserialize it 
    let back = Example::deserialize(json);
    println!("{:?}", back);
}

```

Output of the Example above:

```text
{"opt":null,"code":123,"messages":["Important","Message"]}

Ok(Example { code: 123.0, messages: ["Important", "Message"], opt: None })
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
