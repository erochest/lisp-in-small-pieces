
use nom::branch::alt;
use nom::bytes::complete::{escaped_transform, is_not, tag};
use nom::character::complete::{char, digit1, multispace0, multispace1, none_of};
use nom::combinator::{map, map_res, opt, value};
use nom::error::ErrorKind;
use nom::multi::{many0, many1, separated_list0};
use nom::sequence::{delimited, pair, preceded, tuple};
use nom::Err;
use nom::{IResult, Parser};

use crate::token::Token;

pub fn parse_token_list(input: &str) -> IResult<&str, Vec<Token>> {
    separated_list0(multispace1, parse_token)(input)
}

pub fn parse_token(input: &str) -> IResult<&str, Token> {
    alt((
        comment, cons_list, nil, dot, rational, float, integer, string, sharp_quote, quote, symbol,
    ))(input)
}

fn comment(input: &str) -> IResult<&str, Token> {
    map(
        pair(
            many1(char(';')),
            many0(none_of("\n\r")),
        ),
        |(semis, comment)| Token::Comment {
            depth: semis.len(),
            comment: comment.iter().collect(),
        },
    )(input)
}

fn list_item(input: &str) -> IResult<&str, Token> {
    alt((cons_list, nil, rational, float, integer, string, symbol))(input)
}

fn cons_list(input: &str) -> IResult<&str, Token> {
    map(
        delimited(
            list_start,
            tuple((
                separated_list0(multispace1, list_item),
                opt(tuple((multispace1, dot, multispace1, list_item))),
            )),
            tuple((multispace0, list_end)),
        ),
        |(head, parsed_tail)| {
            if head.is_empty() {
                return Token::EmptyList;
            }
            let mut cons_list: Token = head.into();
            if let Some((_, _, _, tail)) = parsed_tail {
                cons_list.set_last_tail(tail).unwrap();
            }
            cons_list
        },
    )(input)
}

fn nil(input: &str) -> IResult<&str, Token> {
    value(Token::Nil, tag("nil"))(input)
}

fn dot(input: &str) -> IResult<&str, Token> {
    value(Token::Dot, char('.'))(input)
}

fn list_start(input: &str) -> IResult<&str, Token> {
    value(Token::ListStart, char('('))(input)
}

fn list_end(input: &str) -> IResult<&str, Token> {
    value(Token::ListEnd, char(')'))(input)
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

fn sharp_quote(input: &str) -> IResult<&str, Token> {
    map(
        preceded(
            tag("#'"),
            parse_token,
        ),
        |input| Token::Cons {
            head: Box::new(Token::Symbol { value: "function".to_string() }),
            tail: Box::new(Token::Cons {
                head: Box::new(input),
                tail: Box::new(Token::EmptyList),
            }),
        },
    )(input)
}

fn quote(input: &str) -> IResult<&str, Token> {
    map(
        preceded(
            char('\''),
            parse_token,
        ),
        |input| Token::Cons {
            head: Box::new(Token::Symbol { value: "quote".to_string() }),
            tail: Box::new(Token::Cons {
                head: Box::new(input),
                tail: Box::new(Token::EmptyList),
            }),
        },
    )(input)
}

fn symbol(input: &str) -> IResult<&str, Token> {
    map(
        pair(none_of(" .\t\n\r()"), opt(is_not(" \t\n\r()"))),
        |(a, b)| Token::Symbol {
            value: format!("{}{}", a, b.unwrap_or_default()),
        },
    )
    .parse(input)
}
