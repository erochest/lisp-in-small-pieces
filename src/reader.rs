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

#[derive(Debug, Serialize, PartialEq)]
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

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;

    use crate::reader::{Token, read_lisp};

    use Token::*;

    #[test]
    fn test_empty_input() {
        let mut input = "".as_bytes();
        let actual = read_lisp(&mut input);
        assert!(actual.is_ok());
        assert_eq!(actual.unwrap(), vec![]);
    }

    macro_rules! test_parse_token {
        ($name:ident, $input:expr, $token:expr) => {
            #[test]
            fn $name() {
                let mut input = $input.as_bytes();
                let actual = read_lisp(&mut input);
                assert!(actual.is_ok());
                assert_eq!(actual.unwrap(), vec![$token]);
            }
        };
    }

    test_parse_token!(parse_integer, "42", Integer { value: 42 });
    test_parse_token!(parse_symbol, "foobar", Symbol { value: "foobar".to_string() });
    test_parse_token!(parse_float, "   3.14159   ", Float { value: 3.14159 });

}