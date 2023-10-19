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
