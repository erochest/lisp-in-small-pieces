use std::iter::Peekable;
use std::{io::Read, str::FromStr};

use lazy_static::lazy_static;
use regex::Regex;
use serde::Serialize;

use crate::error::{Error, Result};

mod scanner;

use crate::reader::scanner::scan;

use self::scanner::ScanToken;

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
    ListStart,
    ListEnd,
    EmptyList,
    Cons {
        head: Box<Token>,
        tail: Box<Token>,
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
        } else if s == "(" {
            Ok(Token::ListStart)
        } else if s == ")" {
            Ok(Token::ListEnd)
        } else if let Ok(value) = s.parse() {
            Ok(Token::Integer { value })
        } else if let Ok(value) = s.parse() {
            Ok(Token::Float { value })
        } else if let Some(captures) = RE_RATIONAL.captures(s) {
            let numerator = captures[1].parse()?;
            let denominator = captures[2].parse()?;
            Ok(Token::Rational {
                numerator,
                denominator,
            })
        } else if s.starts_with('"') {
            let value = &s[1..s.len() - 1];
            let value = RE_ESCAPE.replace_all(value, "$1");
            let value = value.to_string();
            Ok(Token::String { value })
        } else if !s.is_empty() {
            Ok(Token::Symbol {
                value: s.to_string(),
            })
        } else {
            Err(Error::TokenParseError(s.to_string()))
        }
    }
}

fn is_list_end(opt: Option<&Result<Token>>) -> bool {
    if let Some(r) = opt {
        if let Ok(t) = r {
            return *t == Token::ListEnd
        }
    } 
    false
}

fn read_list_end(tokens: &mut Peekable<impl Iterator<Item = Result<Token>>>) -> Result<Token> {
    if let Some(head) = tokens.next() {
        let head = head?;
        if head == Token::ListEnd {
            return Ok(Token::EmptyList)
        // } else if head == (Token::Symbol { value: ".".to_string() }) {
            // return Ok(Token::Cons { head, tail: () })
        }
    }
    Err(Error::TokenParseError("()".to_string()))
}

pub fn read_lisp<R: Read>(reader: &mut R) -> Result<Vec<Token>> {
    let mut tokens = scan(reader)?
        .map(|s| {
            s.get_string()
                .ok_or_else(|| Error::TokenParseError(String::new()))
                .and_then(|s| Token::from_str(&s))
        })
        .peekable();
    let mut buffer = Vec::new();

    loop {
        if let Some(token) = tokens.next() {
            let token = token?;
            if token == Token::ListStart {
                let list_token = read_list_end(&mut tokens)?;
                buffer.push(list_token);
            } else {
                buffer.push(token);
            }
        } else {
            break;
        }
    }

    Ok(buffer)
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use pretty_assertions::assert_eq;

    use crate::{
        error::Error,
        reader::{read_lisp, Token},
    };

    use Token::*;

    #[test]
    fn from_str_error() {
        let input = "";
        let actual = Token::from_str(input);
        assert!(actual.is_err());
        let err = actual.unwrap_err();
        assert!(match err {
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
    test_from_str_input!(
        from_str_symbol,
        "foobar",
        Symbol {
            value: "foobar".to_string()
        }
    );
    test_from_str_input!(from_str_float, "3.14159", Float { value: 3.14159 });
    test_from_str_input!(
        from_str_rational,
        "2/3",
        Rational {
            numerator: 2,
            denominator: 3
        }
    );
    test_from_str_input!(
        from_str_empty_string,
        "\"\"",
        String {
            value: "".to_string()
        }
    );
    test_from_str_input!(
        from_str_string,
        "\"Hello, World!\"",
        String {
            value: "Hello, World!".to_string()
        }
    );
    test_from_str_input!(
        from_str_string_escaped,
        "\"Hello, \\\"World!\\\"\"",
        String {
            value: "Hello, \"World!\"".to_string()
        }
    );
    test_from_str_input!(from_str_nil, "nil", Nil);
    test_from_str_input!(from_str_list_start, "(", ListStart);
    test_from_str_input!(from_str_list_end, ")", ListEnd);

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

    test_parse_input!(
        parse_sequence,
        "42 13 99",
        Integer { value: 42 },
        Integer { value: 13 },
        Integer { value: 99 }
    );
    test_parse_input!(parse_empty_cons, "()", EmptyList);
    // test_parse_input!(parse_dotted_cons, "(13 . 42)", Cons { head: Box::new(Integer { value: 13 }), tail: Box::new(Integer { value: 42 })});

// TODO: parse a cons list (`(42 43 44)`)
// TODO: parse a quoted symbol (`'foobar`)
// TODO: parse a quoted list (`'(+ 1 3)`)
// TODO: parse a quoted function name (`#'foobar`)
// TODO: parse comments
}
