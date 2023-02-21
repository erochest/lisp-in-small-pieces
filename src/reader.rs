use std::io::Read;

use serde::Serialize;

use crate::error::Result;

// TODO: parse a rational number (`2/3`)
// TODO: parse a string (`"string with spaces"`)
// TODO: parse nil (`nil`)
// TODO: parse an empty list (`()`)
// TODO: parse a dotted-cons cell (`(42 . 43)`)
// TODO: parse a cons list (`(42 43 44)`)
// TODO: parse a quoted symbol (`'foobar`)
// TODO: parse a quoted list (`'(+ 1 3)`)
// TODO: parse a quoted function name (`#'foobar`)
// TODO: parse comments

#[derive(Debug, Serialize)]
#[serde(tag = "type")]
pub enum Token {
    Integer {
        value: isize,
    },
    Float {
        value: f64,
    },
    Symbol {
        value: String,
    },
}

pub fn read_lisp<R: Read>(reader: &mut R) -> Result<Vec<Token>> {
    let mut buffer = String::new();
    reader.read_to_string(&mut buffer)?;
    let buffer = buffer.trim();

    if let Ok(value) = buffer.parse() {
        Ok(vec![Token::Integer { value }])
    } else if let Ok(value) = buffer.parse() {
        Ok(vec![Token::Float { value }])
    } else if buffer.len() > 0 {
        Ok(vec![Token::Symbol { value: buffer.to_string() }])
    } else {
        Ok(vec![])
    }
}