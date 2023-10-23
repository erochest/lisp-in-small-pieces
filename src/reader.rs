use std::io::BufRead;
use std::str::FromStr;

use nom::Finish;

use crate::error::{Error, Result};
use crate::token::Token;

mod parser;

use self::parser::{parse_token, parse_token_list};

impl FromStr for Token {
    type Err = Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        match parse_token(s).finish() {
            Ok((_remaining, token)) => Ok(token),
            Err(nom::error::Error { input, code }) => {
                Err(Error::ParseError(input.to_string(), code))
            }
        }
    }
}

impl From<i64> for Token {
    fn from(value: i64) -> Self {
        Token::Integer { value }
    }
}

impl From<f64> for Token {
    fn from(value: f64) -> Self {
        Token::Float { value }
    }
}

impl From<(i64, i64)> for Token {
    fn from(value: (i64, i64)) -> Self {
        Token::Rational {
            numerator: value.0,
            denominator: value.1,
        }
    }
}

impl From<&str> for Token {
    fn from(value: &str) -> Self {
        Token::String {
            value: value.to_string(),
        }
    }
}

impl From<String> for Token {
    fn from(value: String) -> Self {
        Token::String { value }
    }
}

impl<T: Into<Token>> From<Vec<T>> for Token {
    fn from(value: Vec<T>) -> Self {
        let mut cursor = Token::EmptyList;
        for item in value.into_iter().rev() {
            cursor = Token::Cons {
                head: Box::new(item.into()),
                tail: Box::new(cursor),
            };
        }
        cursor
    }
}

impl From<&[Token]> for Token {
    fn from(value: &[Token]) -> Self {
        let mut cursor = Token::EmptyList;
        for item in value.iter().rev() {
            cursor = Token::Cons {
                head: Box::new(item.to_owned()),
                tail: Box::new(cursor),
            };
        }
        cursor
    }
}

pub fn read_lisp<R: BufRead>(reader: &mut R) -> Result<Vec<Token>> {
    let mut buffer = String::new();
    reader.read_to_string(&mut buffer)?;
    parse_token_list(&buffer)
        .map(|(_, token)| token)
        .map_err(Error::from)
}

#[cfg(test)]
mod tests;
