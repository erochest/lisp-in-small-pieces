use std::fmt::Debug;

use nom::branch::alt;
use nom::bytes::complete::escaped_transform;
use nom::bytes::complete::is_not;
use nom::bytes::complete::tag;
use nom::bytes::complete::take_while1;
use nom::character::complete::char;
use nom::character::complete::digit1;
use nom::character::complete::none_of;
use nom::combinator::map;
use nom::combinator::map_res;
use nom::combinator::opt;
use nom::combinator::value;
use nom::error::ErrorKind;
use nom::multi::many1;
use nom::number;
use nom::sequence::delimited;
use nom::sequence::tuple;
use nom::Err;
use nom::{IResult, Parser};

use crate::token::Token;

pub fn parse_token(input: &str) -> IResult<&str, Token> {
    alt((rational, float, integer, string, symbol))(input)
}

fn rational(input: &str) -> IResult<&str, Token> {
    map_res(tuple((integer, char('/'), integer)), |(n, _, d)| {
        if let (Token::Integer { value: n }, Token::Integer { value: d }) = (n, d) {
            Ok(Token::Rational {
                numerator: n,
                denominator: d,
            })
        } else {
            Err(Err::Error((input, ErrorKind::Fail)))
        }
    })(input)
}

fn float(input: &str) -> IResult<&str, Token> {
    map_res(
        tuple((
            digit1,
            tag("."),
            digit1,
            opt(tuple((
                alt((tag("e"), tag("E"))),
                opt(alt((tag("+"), tag("-")))),
                digit1,
            ))),
        )),
        |(a, _, b, c)| {
            let mut s = String::from(a);
            s.push('.');
            s.push_str(b);
            if let Some((_, sign, exp)) = c {
                s.push('e');
                if let Some(sign) = sign {
                    s.push_str(sign);
                }
                s.push_str(exp);
            }
            s.parse().map(|f| Token::Float { value: f })
        },
    )(input)
}

fn integer(input: &str) -> IResult<&str, Token> {
    map_res(digit1, |input: &str| {
        input.parse().map(|i| Token::Integer { value: i })
    })(input)
}

fn string(input: &str) -> IResult<&str, Token> {
    map(
        delimited(
            char('"'),
            opt(escaped_transform(
                none_of("\\\"\n\r\t"),
                '\\',
                alt((
                    value('\\', char('\\')),
                    value('"', char('"')),
                    value('\'', char('\'')),
                    value('\n', char('n')),
                    value('\r', char('r')),
                    value('\t', char('t')),
                    // TODO: add unicode and other character escapes
                )),
            )),
            char('"'),
        ),
        |input| Token::String {
            value: input.unwrap_or_default().to_string(),
        },
    )(input)
}

fn symbol(input: &str) -> IResult<&str, Token> {
    map(is_not(" \t\n\r()"), |input: &str| Token::Symbol {
        value: input.to_string(),
    })
    .parse(input)
}

// pub struct Parser<'a, T> {
//     input: Box<dyn Iterator<Item = T> + 'a>,
//     buffer: Vec<T>,
// }

// impl<'a, T> Parser<'a, T>
// where
//     T: Parseable + Debug,
// {
//     pub fn new<I: Iterator<Item = T> + 'a>(input: I) -> Self {
//         Parser {
//             input: Box::new(input),
//             buffer: Vec::new(),
//         }
//     }

//     pub fn parse(mut self) -> Vec<T> {
//         let input = self.input;
//         for token in input {
//             self.buffer.push(token);
//             while reduce_buffer(&mut self.buffer) {}
//         }
//         self.buffer
//     }
// }

// fn reduce_buffer<T: Parseable + Debug>(buffer: &mut Vec<T>) -> bool {
//     if let Some((to_replace, production)) = T::propose_reduction(buffer) {
//         buffer.truncate(buffer.len() - to_replace);
//         buffer.push(production);
//         true
//     } else {
//         false
//     }
// }

// pub trait Parseable {
//     fn propose_reduction(buffer: &[Self]) -> Option<(usize, Self)>
//     where
//         Self: Sized;
// }

// #[cfg(test)]
// mod tests {
//     use super::{Parseable, Parser};

//     use pretty_assertions::assert_eq;

//     #[derive(Debug, PartialEq, Clone)]
//     enum Calculator {
//         Number(isize),
//         Plus,
//         Minus,
//         Multiply,
//         Divide,
//         LParen,
//         RParen,

//         // Not sure about having these in with the low-level token enum.
//         // For one, we could get better type support

//         // E -> T [+-] T
//         Expr(Box<Calculator>, Box<Calculator>, Box<Calculator>),
//         // T -> F [*/] F
//         Term(Box<Calculator>, Box<Calculator>, Box<Calculator>),
//         // F -> I
//         // # F -> -F
//         // F -> ( E )
//         // F -> ( T )
//         Factor(Box<Calculator>),
//     }

//     impl Calculator {
//         fn is_number(&self) -> bool {
//             match self {
//                 Number(_) => true,
//                 _ => false,
//             }
//         }

//         fn is_expr(&self) -> bool {
//             match self {
//                 Expr(_, _, _) => true,
//                 _ => false,
//             }
//         }

//         fn is_term(&self) -> bool {
//             match self {
//                 Term(_, _, _) => true,
//                 _ => false,
//             }
//         }

//         fn is_factor(&self) -> bool {
//             match self {
//                 Factor(_) => true,
//                 _ => false,
//             }
//         }

//         fn is_term_op(&self) -> bool {
//             match self {
//                 Multiply | Divide => true,
//                 _ => false,
//             }
//         }

//         fn is_expr_op(&self) -> bool {
//             match self {
//                 Plus | Minus => true,
//                 _ => false,
//             }
//         }
//     }

//     impl Parseable for Calculator {
//         fn propose_reduction(buffer: &[Self]) -> Option<(usize, Self)>
//         where
//             Self: Sized,
//         {
//             if let Some(last) = buffer.last() {
//                 if last.is_number() {
//                     return Some((1, Factor(Box::new(last.clone()))));
//                 }
//             }
//             if buffer.len() >= 3 {
//                 if let Some(end) = buffer.get(buffer.len() - 3..buffer.len()) {
//                     if end[0].is_factor() && end[1].is_term_op() && end[2].is_factor() {
//                         let f1 = end[0].clone();
//                         let op = end[1].clone();
//                         let f2 = end[2].clone();
//                         return Some((3, Term(Box::new(f1), Box::new(op), Box::new(f2))));
//                     }
//                     if end[0].is_term()
//                         && (end[1].is_expr_op() || end[1].is_term())
//                         && end[2].is_term()
//                     {
//                         let f1 = end[0].clone();
//                         let op = end[1].clone();
//                         let f2 = end[2].clone();
//                         return Some((3, Expr(Box::new(f1), Box::new(op), Box::new(f2))));
//                     }
//                     if end[0] == LParen && end[1].is_expr() && end[2] == RParen {
//                         let expr = end[1].clone();
//                         return Some((3, Factor(Box::new(expr))));
//                     }
//                 }
//             }
//             None
//         }
//     }

//     use Calculator::*;

//     #[test]
//     fn test_parse_empty_does_nothing() {
//         let input: Vec<Calculator> = vec![];
//         let input = input.into_iter();
//         let parser = Parser::new(input);

//         let result = parser.parse();
//         assert!(result.is_empty());
//     }

//     #[test]
//     fn test_parse_replaces_single_things() {
//         let input = vec![Number(13)];
//         let input = input.into_iter();
//         let parser = Parser::new(input);

//         let result = parser.parse();

//         assert_eq!(result.len(), 1);
//         assert_eq!(result[0], Factor(Box::new(Number(13))))
//     }

//     #[test]
//     fn test_parse_replaces_lists() {
//         let input = vec![Number(13), Multiply, Number(42)];
//         let input = input.into_iter();
//         let parser = Parser::new(input);

//         let result = parser.parse();

//         assert_eq!(result.len(), 1);
//         assert_eq!(
//             result[0],
//             Term(
//                 Box::new(Factor(Box::new(Number(13)))),
//                 Box::new(Multiply),
//                 Box::new(Factor(Box::new(Number(42)))),
//             )
//         );
//     }
// }
