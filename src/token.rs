use serde::Serialize;

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
    Comment,
}

impl Token {
    pub fn is_cons(&self) -> bool {
        matches!(self, Token::Cons { .. })
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_token() {
        let token = Token::Integer { value: 1 };
        assert_eq!(token, Token::Integer { value: 1 });
    }

    // Test a new method called `is_cons` that returns true if the token is a cons cell.
    #[test]
    fn test_is_cons() {
        let token = Token::Cons {
            head: Box::new(Token::Integer { value: 1 }),
            tail: Box::new(Token::Integer { value: 2 }),
        };
        assert_eq!(token.is_cons(), true);
    }
}
