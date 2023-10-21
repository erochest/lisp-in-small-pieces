use serde::Serialize;

use crate::error::{Error, Result};

#[derive(Debug, Serialize, PartialEq, Clone)]
#[serde(tag = "type")]
pub enum Token {
    Integer { value: i64 },
    Float { value: f64 },
    Rational { numerator: i64, denominator: i64 },
    String { value: String },
    Symbol { value: String },
    ListStart,
    ListEnd,
    EmptyList,
    Cons { head: Box<Token>, tail: Box<Token> },
    Nil,
    Dot,
    Comment,
}

impl Token {
    pub fn is_cons(&self) -> bool {
        matches!(self, Token::Cons { .. })
    }

    pub fn set_tail(&mut self, tail: Token) -> Result<()> {
        match self {
            Token::Cons { tail: t, .. } => {
                *t = Box::new(tail);
                Ok(())
            }
            _ => Err(Error::InvalidTokenOperation(
                "Token is not a cons".to_string(),
            )),
        }
    }

    pub fn set_last_tail(&mut self, tail: Token) -> Result<()> {
        match self {
            Token::Cons { tail: t, .. } => match **t {
                Token::Cons { .. } => t.set_last_tail(tail),
                Token::EmptyList => {
                    let mut tail = tail.clone();
                    std::mem::swap(&mut tail, t);
                    Ok(())
                }
                _ => Err(Error::InvalidTokenOperation(
                    "Token is not a cons".to_string(),
                )),
            },
            _ => Err(Error::InvalidTokenOperation(
                "Token is not a cons".to_string(),
            )),
        }
    }
}

#[cfg(test)]
mod test {
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn test_token() {
        let token = Token::Integer { value: 1 };
        assert_eq!(token, Token::Integer { value: 1 });
    }

    #[test]
    fn test_is_cons() {
        let token = Token::Cons {
            head: Box::new(Token::Integer { value: 1 }),
            tail: Box::new(Token::Integer { value: 2 }),
        };
        assert_eq!(token.is_cons(), true);
    }

    #[test]
    fn test_is_cons_not() {
        let token = Token::Integer { value: 1 };
        assert_eq!(token.is_cons(), false);
    }

    #[test]
    fn test_set_tail_not_cons() {
        let mut token = Token::Integer { value: 1 };
        let result = token.set_tail(Token::Integer { value: 2 });
        assert_eq!(result.is_err(), true);
    }

    #[test]
    fn test_set_tail_node() {
        let mut token = Token::Cons {
            head: Box::new(Token::Integer { value: 1 }),
            tail: Box::new(Token::Integer { value: 2 }),
        };
        let result = token.set_tail(Token::Integer { value: 3 });
        assert_eq!(result.is_ok(), true);
        assert_eq!(
            token,
            Token::Cons {
                head: Box::new(Token::Integer { value: 1 }),
                tail: Box::new(Token::Integer { value: 3 }),
            }
        );
    }

    #[test]
    fn test_set_last_tail() {
        let mut token = Token::Cons {
            head: Box::new(Token::Integer { value: 1 }),
            tail: Box::new(Token::Cons {
                head: Box::new(Token::Integer { value: 2 }),
                tail: Box::new(Token::EmptyList),
            }),
        };
        let result = token.set_last_tail(Token::Integer { value: 4 });
        assert_eq!(result.is_ok(), true);
        assert_eq!(
            token,
            Token::Cons {
                head: Box::new(Token::Integer { value: 1 }),
                tail: Box::new(Token::Cons {
                    head: Box::new(Token::Integer { value: 2 }),
                    tail: Box::new(Token::Integer { value: 4 }),
                }),
            }
        );
    }
}
