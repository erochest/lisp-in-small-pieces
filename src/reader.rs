use std::io::BufRead;
use std::str::FromStr;

use nom::Finish;

use crate::error::{Error, Result};

mod parser;
mod scanner;

use crate::token::Token;

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
mod tests {
    use std::str::FromStr;

    use pretty_assertions::assert_eq;

    use crate::reader::{read_lisp, Token};

    use Token::*;

    #[test]
    fn from_i64() {
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
    // test_from_str_input!(from_str_list_start, "(", ListStart);
    // test_from_str_input!(from_str_list_end, ")", ListEnd);

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
    test_parse_input!(parse_empty_cons_space, "( )", EmptyList);
    test_parse_input!(
        parse_dotted_cons,
        "(13 . 42)",
        Cons {
            head: Box::new(Integer { value: 13 }),
            tail: Box::new(Integer { value: 42 })
        }
    );
    test_parse_input!(
        parse_list,
        "(42 43 44)",
        vec![Into::<Token>::into(42i64), 43.into(), 44.into()].into()
    );
    // also a (1 2 . (3 4)) test
    test_parse_input!(
        parse_dotted_list_notation,
        "(1 2 . (3 4))",
        vec![Into::<Token>::into(1i64), 2.into(), 3.into(), 4.into()].into()
    );
    test_parse_input!(
        parse_symbol_with_digits,
        "symbol-42",
        Symbol {
            value: "symbol-42".to_string()
        }
    );
    test_parse_input!(
        parse_symbol_list,
        "(symbol-42 symbol-43 symbol-44)",
        vec![
            Token::Symbol {
                value: "symbol-42".to_string()
            },
            Token::Symbol {
                value: "symbol-43".to_string()
            },
            Token::Symbol {
                value: "symbol-44".to_string()
            }
        ]
        .into()
    );
    test_parse_input!(
        parse_embedded_lists,
        "(+ 7 (- 10 3))",
        vec![
            Symbol {
                value: "+".to_string()
            },
            7i64.into(),
            vec![
                Symbol {
                    value: "-".to_string()
                },
                Into::<Token>::into(10i64),
                3.into()
            ]
            .into(),
        ]
        .into()
    );

    test_parse_input!(
        parse_operators,
        "+ - 42",
        Symbol {
            value: "+".to_string()
        },
        Symbol {
            value: "-".to_string()
        },
        Integer { value: 42 }
    );

    test_parse_input!(
        parse_quoted_symbol,
        "'foobar",
        vec![
            Symbol {
                value: "quote".to_string()
            },
            Symbol {
                value: "foobar".to_string()
            },
        ]
        .into()
    );

    test_parse_input!(
        parse_quoted_list,
        "'(+ 1 3)",
        vec![
            Symbol {
                value: "quote".to_string()
            },
            vec![
                Symbol {
                    value: "+".to_string()
                },
                1.into(),
                3.into(),
            ]
            .into(),
        ]
        .into()
    );

    test_parse_input!(
        parse_quoted_function,
        "#'foo-bar",
        vec![
            Symbol {
                value: "function".to_string()
            },
            Symbol {
                value: "foo-bar".to_string()
            },
        ]
        .into()
    );

    test_parse_input!(
        parse_comments,
        "something ; commented\nsomething-else",
        Symbol {
            value: "something".to_string()
        },
        Comment {
            depth: 1,
            comment: " commented".to_string(),
        },
        Symbol {
            value: "something-else".to_string()
        }
    );
}
