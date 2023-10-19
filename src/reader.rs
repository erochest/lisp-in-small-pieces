use std::{io::Read, str::FromStr};

use lazy_static::lazy_static;
use nom::Finish;
use regex::Regex;
use serde::Serialize;

use crate::error::{Error, Result};

mod parser;
mod scanner;

use crate::token::Token;

use self::parser::parse_token;

// TODO: factor out tokens and things (token, from<_>, parseable)
// TODO: ctors of different kinds of tokens

impl FromStr for Token {
    type Err = Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        match parse_token(s).finish() {
            Ok((_remaining, token)) => Ok(token),
            Err(nom::error::Error { input, code }) => {
                Err(Error::ParseError(input.to_string(), code))
            }
        }
        // lazy_static! {
        //     static ref RE_RATIONAL: Regex = Regex::new(r"([+-]?\d+)/(\d+)").unwrap();
        //     static ref RE_STRING: Regex = Regex::new("\"((\\\\\")|[^\"]*)\"").unwrap();
        //     static ref RE_ESCAPE: Regex = Regex::new(r"\\(.)").unwrap();
        // }

        // if s == "nil" {
        //     Ok(Token::Nil)
        // } else if s == "(" {
        //     Ok(Token::ListStart)
        // } else if s == ")" {
        //     Ok(Token::ListEnd)
        // } else if s.starts_with(';') {
        //     Ok(Token::Comment)
        // } else if let Ok(value) = s.parse() {
        //     Ok(Token::Integer { value })
        // } else if let Ok(value) = s.parse() {
        //     Ok(Token::Float { value })
        // } else if let Some(captures) = RE_RATIONAL.captures(s) {
        //     let numerator = captures[1].parse()?;
        //     let denominator = captures[2].parse()?;
        //     Ok(Token::Rational {
        //         numerator,
        //         denominator,
        //     })
        // } else if s.starts_with('"') {
        //     let value = &s[1..s.len() - 1];
        //     let value = RE_ESCAPE.replace_all(value, "$1");
        //     let value = value.to_string();
        //     Ok(Token::String { value })
        // } else if !s.is_empty() {
        //     Ok(Token::Symbol {
        //         value: s.to_string(),
        //     })
        // } else {
        //     Err(Error::TokenParseError(s.to_string()))
        // }
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

impl From<(isize, isize)> for Token {
    fn from(value: (isize, isize)) -> Self {
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

fn parse_empty_list(buffer: &[Token]) -> Option<(usize, Token)> {
    let buffer_len = buffer.len();
    if buffer_len >= 2 {
        if let Some(tail) = buffer.get(buffer_len - 2..) {
            if tail[0].is_list_start() && tail[1].is_list_end() {
                return Some((2, Token::EmptyList));
            }
        }
    }
    None
}

fn parse_dotted_cell(buffer: &[Token]) -> Option<(usize, Token)> {
    let buffer_len = buffer.len();
    if buffer_len >= 5 {
        if let Some(tail) = buffer.get(buffer_len - 5..) {
            let dot = Token::Symbol {
                value: ".".to_string(),
            };
            if tail[0].is_list_start() && tail[2] == dot && tail[4].is_list_end() {
                return Some((
                    5,
                    Token::Cons {
                        head: Box::new(tail[1].clone()),
                        tail: Box::new(tail[3].clone()),
                    },
                ));
            }
        }
    }
    None
}

fn parse_list(buffer: &[Token]) -> Option<(usize, Token)> {
    let buffer_len = buffer.len();
    if buffer_len > 2 && buffer[buffer_len - 1] == Token::ListEnd {
        for (i, item) in buffer.iter().rev().enumerate() {
            if *item == Token::ListStart {
                let start = buffer_len - i;
                let list: Token = buffer[start..buffer_len - 1].into();
                return Some((i + 1, list));
            }
        }
    }
    None
}

fn parse_quoted(buffer: &[Token]) -> Option<(usize, Token)> {
    let buffer_len = buffer.len();
    if buffer_len >= 2
        && buffer[buffer_len - 2]
            == (Token::Symbol {
                value: "'".to_string(),
            })
        && buffer[buffer_len - 1] != Token::ListStart
    {
        return Some((
            2,
            Token::Cons {
                head: Box::new(Token::Symbol {
                    value: "quote".to_string(),
                }),
                tail: Box::new(Token::Cons {
                    head: Box::new(buffer[buffer_len - 1].clone()),
                    tail: Box::new(Token::EmptyList),
                }),
            },
        ));
    }
    None
}

fn parse_sharp_quote(buffer: &[Token]) -> Option<(usize, Token)> {
    let buffer_len = buffer.len();
    if buffer_len >= 2
        && buffer[buffer_len - 2]
            == (Token::Symbol {
                value: "#'".to_string(),
            })
    {
        return Some((
            2,
            Token::Cons {
                head: Box::new(Token::Symbol {
                    value: "function".to_string(),
                }),
                tail: Box::new(Token::Cons {
                    head: Box::new(buffer[buffer_len - 1].clone()),
                    tail: Box::new(Token::EmptyList),
                }),
            },
        ));
    }
    None
}

// impl Parseable for Token {
//     fn propose_reduction(buffer: &[Self]) -> Option<(usize, Self)>
//     where
//         Self: Sized,
//     {
//         parse_empty_list(buffer)
//             .or_else(|| parse_dotted_cell(buffer))
//             .or_else(|| parse_list(buffer))
//             .or_else(|| parse_sharp_quote(buffer))
//             // This one probably needs to be last so the quoted thing is fully recognized.
//             .or_else(|| parse_quoted(buffer))
//     }
// }

impl Token {
    fn is_list_start(&self) -> bool {
        matches!(self, Token::ListStart)
    }

    fn is_list_end(&self) -> bool {
        matches!(self, Token::ListEnd)
    }

    fn is_comment(&self) -> bool {
        matches!(self, Token::Comment)
    }
}

pub fn read_lisp<R: Read>(reader: &mut R) -> Result<Vec<Token>> {
    todo!()
}

// pub fn read_lisp<R: Read>(reader: &mut R) -> Result<Vec<Token>> {
//     let tokens = scan(reader)?
//         .map(|s| {
//             s.get_string()
//                 .ok_or_else(|| Error::TokenParseError(String::new()))
//                 .and_then(|s| Token::from_str(&s))
//         })
//         .filter(|r| match r {
//             Ok(ref t) => !t.is_comment(),
//             Err(_) => true,
//         })
//         .collect::<Result<Vec<_>>>()?
//         .into_iter();
//     let parser = Parser::new(tokens);

//     let result = parser.parse();

//     Ok(result)
// }

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use pretty_assertions::assert_eq;

    use crate::{
        error::Error,
        reader::{Read, Token},
    };

    use Token::*;

    // #[test]
    // fn from_str_error() {
    //     let input = "";
    //     let actual = Token::from_str(input);
    //     assert!(actual.is_err());
    //     let err = actual.unwrap_err();
    //     assert!(match err {
    //         Error::TokenParseError(_) => true,
    //         _ => false,
    //     });
    // }

    #[test]
    fn from_isize() {
        assert_eq!(Integer { value: 99 }, 99i64.into());
    }

    #[test]
    fn from_f64() {
        assert_eq!(Float { value: 3.14159 }, 3.14159f64.into());
    }

    #[test]
    fn from_int_pair() {
        assert_eq!(
            Rational {
                numerator: 13,
                denominator: 74
            },
            (13, 74).into()
        );
    }

    #[test]
    fn from_str() {
        assert_eq!(
            String {
                value: "a string".to_string()
            },
            "a string".into()
        );
    }

    #[test]
    fn from_string() {
        assert_eq!(
            String {
                value: "another string".to_string()
            },
            "another string".to_string().into()
        );
    }

    #[test]
    fn from_token_vec() {
        let input: Vec<Token> = vec![42.into(), "+".into(), 99.into()];
        let expected = Cons {
            head: Box::new(42.into()),
            tail: Box::new(Cons {
                head: Box::new("+".into()),
                tail: Box::new(Cons {
                    head: Box::new(99.into()),
                    tail: Box::new(EmptyList),
                }),
            }),
        };
        assert_eq!(expected, input.into());
    }

    #[test]
    fn from_token_slice() {
        let input: &[Token] = &[42.into(), "+".into(), 99.into()];
        let expected = Cons {
            head: Box::new(42.into()),
            tail: Box::new(Cons {
                head: Box::new("+".into()),
                tail: Box::new(Cons {
                    head: Box::new(99.into()),
                    tail: Box::new(EmptyList),
                }),
            }),
        };
        assert_eq!(expected, input.into());
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
    // test_from_str_input!(
    //     from_str_rational,
    //     "2/3",
    //     Rational {
    //         numerator: 2,
    //         denominator: 3
    //     }
    // );
    // test_from_str_input!(
    //     from_str_empty_string,
    //     "\"\"",
    //     String {
    //         value: "".to_string()
    //     }
    // );
    // test_from_str_input!(
    //     from_str_string,
    //     "\"Hello, World!\"",
    //     String {
    //         value: "Hello, World!".to_string()
    //     }
    // );
    // test_from_str_input!(
    //     from_str_string_escaped,
    //     "\"Hello, \\\"World!\\\"\"",
    //     String {
    //         value: "Hello, \"World!\"".to_string()
    //     }
    // );
    // test_from_str_input!(from_str_nil, "nil", Nil);
    // test_from_str_input!(from_str_list_start, "(", ListStart);
    // test_from_str_input!(from_str_list_end, ")", ListEnd);

    // macro_rules! test_parse_input {
    //     ($name:ident, $input:expr, $( $token:expr ),*) => {
    //         #[test]
    //         fn $name() {
    //             let mut input = $input.as_bytes();
    //             let actual = read_lisp(&mut input);
    //             assert!(actual.is_ok());
    //             assert_eq!(actual.unwrap(), vec![$( $token, )*]);
    //         }
    //     };
    // }

    // test_parse_input!(
    //     parse_sequence,
    //     "42 13 99",
    //     Integer { value: 42 },
    //     Integer { value: 13 },
    //     Integer { value: 99 }
    // );
    // test_parse_input!(parse_empty_cons, "()", EmptyList);
    // test_parse_input!(
    //     parse_dotted_cons,
    //     "(13 . 42)",
    //     Cons {
    //         head: Box::new(Integer { value: 13 }),
    //         tail: Box::new(Integer { value: 42 })
    //     }
    // );
    // test_parse_input!(
    //     parse_list,
    //     "(42 43 44)",
    //     vec![Into::<Token>::into(42isize), 43.into(), 44.into()].into()
    // );
    // test_parse_input!(
    //     parse_symbol_list,
    //     "(symbol-42 symbol-43 symbol-44)",
    //     vec![Into::<Token>::into(42isize), 43.into(), 44.into()].into()
    // );
    // test_parse_input!(
    //     parse_embedded_lists,
    //     "(+ 7 (- 10 3))",
    //     vec![
    //         Symbol {
    //             value: "+".to_string()
    //         },
    //         7isize.into(),
    //         vec![
    //             Symbol {
    //                 value: "-".to_string()
    //             },
    //             Into::<Token>::into(10isize),
    //             3.into()
    //         ]
    //         .into(),
    //     ]
    //     .into()
    // );

    // test_parse_input!(
    //     parse_quoted_symbol,
    //     "'foobar",
    //     vec![
    //         Symbol {
    //             value: "quote".to_string()
    //         },
    //         Symbol {
    //             value: "foobar".to_string()
    //         },
    //     ]
    //     .into()
    // );

    // test_parse_input!(
    //     parse_quoted_list,
    //     "'(+ 1 3)",
    //     vec![
    //         Symbol {
    //             value: "quote".to_string()
    //         },
    //         vec![
    //             Symbol {
    //                 value: "+".to_string()
    //             },
    //             1.into(),
    //             3.into(),
    //         ]
    //         .into(),
    //     ]
    //     .into()
    // );

    // test_parse_input!(
    //     parse_quoted_function,
    //     "#'foo-bar",
    //     vec![
    //         Symbol {
    //             value: "function".to_string()
    //         },
    //         Symbol {
    //             value: "foo-bar".to_string()
    //         },
    //     ]
    //     .into()
    // );

    // test_parse_input!(
    //     parse_comments,
    //     "something ; commented\nsomething-else",
    //     Symbol {
    //         value: "something".to_string()
    //     },
    //     Symbol {
    //         value: "something-else".to_string()
    //     }
    // );
}
