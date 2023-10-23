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
