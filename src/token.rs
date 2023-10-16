use serde::Serialize;

#[derive(Debug, Serialize, PartialEq, Clone)]
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
    Comment,
}
