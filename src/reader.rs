use std::io::Read;

use serde::Serialize;

use crate::error::Result;

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