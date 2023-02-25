use std::{io::Read, str::FromStr};

use lazy_static::lazy_static;
use regex::Regex;
use serde::Serialize;

use crate::error::{Result, Error};

mod scanner;

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
        if let Ok(value) = s.parse() {
            Ok(Token::Integer { value })
        } else {
            Err(Error::TokenParseError(s.to_string()))
        }
    }
}

pub fn read_lisp<R: Read>(reader: &mut R) -> Result<Vec<Token>> {
    lazy_static! {
        static ref RE_RATIONAL: Regex = Regex::new(r"([+-]?\d+)/(\d+)").unwrap();
        static ref RE_STRING: Regex = Regex::new("\"((\\\\\")|[^\"]*)\"").unwrap();
    }

    let mut buffer = String::new();
    reader.read_to_string(&mut buffer)?;
    buffer = buffer.trim().to_string();

    // let mut tokens = Vec::new();
    // let mut i = 0;
    // let mut current = String::new();

    // for c in buffer.chars() {
    //     if c.is_whitespace() {
    //         continue;
    //     }

    // }

    // return Ok(tokens);

    if buffer == "nil" {
        Ok(vec![Token::Nil])
    } else if let Ok(value) = buffer.parse() {
        Ok(vec![Token::Integer { value }])
    } else if let Ok(value) = buffer.parse() {
        Ok(vec![Token::Float { value }])
    } else if let Some(captures) = RE_RATIONAL.captures(&buffer) {
        let numerator = captures[1].parse().unwrap();
        let denominator = captures[2].parse().unwrap();
        Ok(vec![Token::Rational { numerator, denominator }])
    } else if let Some(captures) = RE_STRING.find(&buffer) {
        let value = captures.as_str()[1..(captures.end()-1)].to_string();
        Ok(vec![Token::String { value }])
    } else if buffer.len() > 0 {
        Ok(vec![Token::Symbol { value: buffer.to_string() }])
    } else {
        Ok(vec![])
    }
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

    #[test]
    fn from_str_integer() {
        let input = "42";
        let actual = Token::from_str(input);
        assert!(actual.is_ok());
        assert_eq!(actual.unwrap(), Integer { value: 42 });
    }

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

    test_parse_input!(parse_empty, "", );
    test_parse_input!(parse_integer, "42", Integer { value: 42 });
    test_parse_input!(parse_symbol, "foobar", Symbol { value: "foobar".to_string() });
    test_parse_input!(parse_float, "   3.14159   ", Float { value: 3.14159 });
    test_parse_input!(parse_rational, " 2/3 ", Rational { numerator: 2, denominator: 3 });
    test_parse_input!(parse_empty_string, "\"\"", String { value: "".to_string() });
    test_parse_input!(parse_string, " \"Hello, world!\" ", String { value: "Hello, world!".to_string() });
    test_parse_input!(parse_nil, "  nil ", Nil);
    // test_parse_input!(parse_sequence, "42 13 99", Integer { value: 42 }, Integer { value: 13 }, Integer { value: 99 });

}