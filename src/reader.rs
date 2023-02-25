use std::{io::Read, str::FromStr};

use lazy_static::lazy_static;
use regex::Regex;
use serde::Serialize;

use crate::error::{Result, Error};

mod scanner;

use crate::reader::scanner::scan;

// TODO: parse an empty list (`()`)
// TODO: parse a dotted-cons cell (`(42 . 43)`)
// TODO: parse a cons list (`(42 43 44)`)
// TODO: parse a quoted symbol (`'foobar`)
// TODO: parse a quoted list (`'(+ 1 3)`)
// TODO: parse a quoted function name (`#'foobar`)
// TODO: parse comments

#[derive(Debug, Serialize, PartialEq)]
#[serde(tag = "type")]
pub enum Token {
    Integer {
        value: isize,
    },
    Float {
        value: f64,
    },
    Rational {
        numerator: isize,
        denominator: isize,
    },
    String {
        value: String,
    },
    Symbol {
        value: String,
    },
    Nil,
}

impl FromStr for Token {
    type Err = Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        lazy_static! {
            static ref RE_RATIONAL: Regex = Regex::new(r"([+-]?\d+)/(\d+)").unwrap();
            static ref RE_STRING: Regex = Regex::new("\"((\\\\\")|[^\"]*)\"").unwrap();
            static ref RE_ESCAPE: Regex = Regex::new(r"\\(.)").unwrap();
        }

        if s == "nil" {
            Ok(Token::Nil)
        } else if let Ok(value) = s.parse() {
            Ok(Token::Integer { value })
        } else if let Ok(value) = s.parse() {
            Ok(Token::Float { value })
        } else if let Some(captures) = RE_RATIONAL.captures(&s) {
            let numerator = captures[1].parse()?;
            let denominator = captures[2].parse()?;
            Ok(Token::Rational { numerator, denominator })
        } else if s.starts_with('"') {
            let value = &s[1..s.len()-1];
            let value = RE_ESCAPE.replace_all(value, "$1");
            let value = value.to_string();
            Ok(Token::String { value })
        } else if s.len() > 0 {
            Ok(Token::Symbol { value: s.to_string() })
        } else {
            Err(Error::TokenParseError(s.to_string()))
        }
    }
}

pub fn read_lisp<R: Read>(reader: &mut R) -> Result<Vec<Token>> {
    scan(reader)?
    .map(|s| s.get_string().ok_or_else(|| Error::TokenParseError(String::new())).and_then(|s| Token::from_str(&s)))
    .collect()
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use pretty_assertions::assert_eq;

    use crate::{reader::{Token, read_lisp}, error::Error};

    use Token::*;

    #[test]
    fn from_str_error() {
        let input = "";
        let actual = Token::from_str(input);
        assert!(actual.is_err());
        let err = actual.unwrap_err();
        assert!(match  err {
            Error::TokenParseError(_) => true,
            _ => false,
        });
    }

    macro_rules! test_from_str_input {
        ($name:ident, $input:expr, $token:expr) => {
            #[test]
            fn $name() {
                let input = $input;
                let actual = Token::from_str(input);
                assert!(actual.is_ok());
                assert_eq!(actual.unwrap(), $token);
            }
        };
    }

    test_from_str_input!(from_str_integer, "42", Integer { value: 42 });
    test_from_str_input!(from_str_symbol, "foobar", Symbol { value: "foobar".to_string() });
    test_from_str_input!(from_str_float, "3.14159", Float { value: 3.14159 });
    test_from_str_input!(from_str_rational, "2/3", Rational { numerator: 2, denominator: 3 });
    test_from_str_input!(from_str_empty_string, "\"\"", String { value: "".to_string() });
    test_from_str_input!(from_str_string, "\"Hello, World!\"", String { value: "Hello, World!".to_string() });
    test_from_str_input!(from_str_string_escaped, "\"Hello, \\\"World!\\\"\"", String { value: "Hello, \"World!\"".to_string() });
    test_from_str_input!(from_str_nil, "nil", Nil);

    macro_rules! test_parse_input {
        ($name:ident, $input:expr, $( $token:expr ),*) => {
            #[test]
            fn $name() {
                let mut input = $input.as_bytes();
                let actual = read_lisp(&mut input);
                assert!(actual.is_ok());
                assert_eq!(actual.unwrap(), vec![$( $token, )*]);
            }
        };
    }

    // test_parse_input!(parse_sequence, "42 13 99", Integer { value: 42 }, Integer { value: 13 }, Integer { value: 99 });

}