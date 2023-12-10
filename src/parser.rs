use std::collections::HashMap;
use std::str::FromStr;

use crate::error::ParseError;
use crate::values::Values;

#[macro_use]
pub(crate) mod macros {
    macro_rules! expect_byte {
        ($parser:ident) => {{
            if $parser.is_eof() {
                return Err(ParseError::new());
            }

            let ch = $parser.read_byte();
            $parser.bump();
            ch
        }};
    }

    // Expect a sequence of specific bytes in specific order, error otherwise.
    // This is useful for reading the 3 JSON identifiers:
    //
    // - "t" has to be followed by "rue"
    // - "f" has to be followed by "alse"
    // - "n" has to be followed by "ull"
    //
    // Anything else is an error.
    macro_rules! expect_sequence {
        ($parser:ident, $( $ch:pat ),*) => {
            $(
                match expect_byte!($parser) {
                    $ch => {}
                    _   => return $parser.unexpected_character(),
                }
            )*
        }
    }

    // A drop in macro for when we expect to read a byte, but we don't care
    // about any whitespace characters that might occur before it.
    macro_rules! expect_byte_ignore_whitespace {
        ($parser:ident) => {{
            let mut ch = expect_byte!($parser);

            // Don't go straight for the loop, assume we are in the clear first.
            match ch {
                // whitespace
                9..=13 | 32 => loop {
                    match expect_byte!($parser) {
                        9..=13 | 32 => {}
                        next => {
                            ch = next;
                            break;
                        }
                    }
                },
                _ => {}
            }

            ch
        }};
    }

    // Expect to find EOF or just whitespaces leading to EOF after a JSON value
    macro_rules! expect_eof {
        ($parser:ident) => {{
            while !$parser.is_eof() {
                match $parser.read_byte() {
                    9..=13 | 32 => $parser.bump(),
                    _ => {
                        $parser.bump();
                        return $parser.unexpected_character();
                    }
                }
            }
        }};
    }

    // Expect a particular byte to be next. Also available with a variant
    // creates a `match` expression just to ease some pain.
    macro_rules! expect {
        ($parser:ident, $byte:expr) => ({
            let ch = expect_byte_ignore_whitespace!($parser);

            if ch != $byte {
                return $parser.unexpected_character();
            }
        });

        {$parser:ident $(, $byte:pat => $then:expr )*} => ({
            let ch = expect_byte_ignore_whitespace!($parser);
            match ch {
                $(
                    $byte => $then,
                )*
                _ => return $parser.unexpected_character()
            }
        })
    }
}

pub struct Parser {
    byte_ptr: *const u8,
    index: usize,
    length: usize,
}

struct StackBlock(Values, String);

impl<'a> Parser {
    pub fn parse(&mut self) -> Result<Values, ParseError> {
        let mut stack = Vec::with_capacity(3);
        let mut ch = expect_byte_ignore_whitespace!(self);

        'parsing: loop {
            let mut value = match ch {
                b'[' => {
                    ch = expect_byte_ignore_whitespace!(self);

                    if ch != b']' {
                        stack.push(StackBlock(
                            Values::Array(Vec::with_capacity(2)),
                            "UNIMPORTANT".into(),
                        ));
                        continue 'parsing;
                    }

                    Values::Array(Vec::new())
                }
                b'{' => {
                    ch = expect_byte_ignore_whitespace!(self);

                    if ch != b'}' {
                        let mut map = HashMap::with_capacity(3);

                        if ch != b'"' {
                            return self.unexpected_character();
                        }
                        let index = self.expect_string()?;
                        map.insert(index.clone(), Values::Null);
                        expect!(self, b':');
                        stack.push(StackBlock(Values::Struct(map), index));

                        ch = expect_byte_ignore_whitespace!(self);

                        continue 'parsing;
                    }

                    Values::Struct(HashMap::new())
                }
                b'"' => Values::String(self.expect_string()?),
                b'0'..=b'9' => Values::Number(self.expect_number(ch)?),
                b'-' => {
                    let ch = expect_byte!(self);
                    Values::Number(-match ch {
                        b'0'..=b'9' => self.expect_number(ch)?,
                        _ => return self.unexpected_character(),
                    })
                }
                b't' => {
                    expect_sequence!(self, b'r', b'u', b'e');
                    Values::Boolean(true)
                }
                b'f' => {
                    expect_sequence!(self, b'a', b'l', b's', b'e');
                    Values::Boolean(false)
                }
                b'n' => {
                    expect_sequence!(self, b'u', b'l', b'l');
                    Values::Null
                }
                _ => return self.unexpected_character(),
            };

            'popping: loop {
                match stack.last_mut() {
                    None => {
                        expect_eof!(self);

                        return Ok(value);
                    }

                    Some(&mut StackBlock(Values::Array(ref mut array), _)) => {
                        array.insert(0, value);

                        ch = expect_byte_ignore_whitespace!(self);

                        match ch {
                            b',' => {
                                ch = expect_byte_ignore_whitespace!(self);

                                continue 'parsing;
                            }
                            b']' => {}
                            _ => return self.unexpected_character(),
                        }
                    }

                    Some(&mut StackBlock(Values::Struct(ref mut object), ref mut index)) => {
                        object.insert(index.to_string(), value);

                        ch = expect_byte_ignore_whitespace!(self);

                        match ch {
                            b',' => {
                                expect!(self, b'"');
                                let string = self.expect_string()?;
                                object.insert(string.clone(), Values::Null);
                                *index = string;
                                expect!(self, b':');

                                ch = expect_byte_ignore_whitespace!(self);

                                continue 'parsing;
                            }
                            b'}' => {}
                            _ => return self.unexpected_character(),
                        }
                    }

                    _ => unreachable!(),
                }

                value = match stack.pop() {
                    Some(StackBlock(value, _)) => value,
                    None => break 'popping,
                }
            }
        }
    }
    pub fn new(source: &'a str) -> Self {
        Parser {
            byte_ptr: source.as_ptr(),
            index: 0,
            length: source.len(),
        }
    }
    fn expect_string(&mut self) -> Result<String, ParseError> {
        let mut string = String::new();
        loop {
            let char = self.read_byte();
            if char == b'"' {
                self.bump();
                return Ok(string);
            }
            if char == b'\\' {
                self.bump();
                let escaped = expect_byte!(self);
                let escaped = match escaped {
                    b'u' => {
                        // Unicode Characters are not supported
                        continue;
                    }
                    b'"' => b'\"',
                    b'\\' | b'/' => escaped,
                    b'b' => 0x8,
                    b'f' => 0xC,
                    b't' => b'\t',
                    b'r' => b'\r',
                    b'n' => b'\n',
                    _ => return self.unexpected_character(),
                };
                string.push(char::from(escaped));
            } else {
                string.push(char::from(char));
                self.bump();
            }
        }
    }

    fn expect_number(&mut self, mut num: u8) -> Result<f64, ()> {
        let mut string = String::from(char::from(num));

        loop {
            if !self.is_eof() {
                num = self.read_byte();
            }
            match num {
                b'\\' | b' ' | b',' | b']' | b'}' | b'\n' | b'\r' => break,
                _ => {
                    string.push(char::from(num));
                    self.bump();
                }
            }
        }

        f64::from_str(string.as_str()).map_err(|_err| ())
    }

    fn is_eof(&self) -> bool {
        self.index == self.length
    }

    fn read_byte(&self) -> u8 {
        debug_assert!(self.index < self.length, "Reading out of bounds");

        unsafe { *self.byte_ptr.add(self.index) }
    }

    fn bump(&mut self) {
        self.index = self.index.wrapping_add(1);
    }

    fn unexpected_character<T: Sized>(&mut self) -> Result<T, ParseError> {
        Err(ParseError::new())
    }
}
