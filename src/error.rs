use std::error;
use std::fmt;
use std::io;
use std::num::ParseIntError;
use std::result;

use nom::error::ErrorKind;

pub type Result<R> = result::Result<R, Error>;

#[derive(Debug)]
pub enum Error {
    IoError(io::Error),
    SerializationError(serde_json::Error),
    IntParseError(ParseIntError),
    TokenParseError(String),
    ParseError(String, ErrorKind),
    InvalidTokenOperation(String),
}

use Error::*;

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            IoError(ref err) => err.fmt(f),
            SerializationError(ref err) => err.fmt(f),
            IntParseError(ref err) => err.fmt(f),
            TokenParseError(ref input) => write!(f, "token parsing error on {:?}", input),
            ParseError(ref input, ref code) => {
                write!(f, "parse error in {:?}: code {:?}", input, code)
            }
            InvalidTokenOperation(ref msg) => write!(f, "invalid token operation: {}", msg),
        }
    }
}

impl error::Error for Error {}

impl From<io::Error> for Error {
    fn from(value: io::Error) -> Self {
        IoError(value)
    }
}

impl From<serde_json::Error> for Error {
    fn from(value: serde_json::Error) -> Self {
        SerializationError(value)
    }
}

impl From<ParseIntError> for Error {
    fn from(value: ParseIntError) -> Self {
        IntParseError(value)
    }
}

impl From<nom::Err<nom::error::Error<&str>>> for Error {
    fn from(value: nom::Err<nom::error::Error<&str>>) -> Self {
        match value {
            nom::Err::Incomplete(_) => unreachable!(),
            nom::Err::Error(ref err) | nom::Err::Failure(ref err) => {
                ParseError(err.input.to_string(), err.code)
            }
        }
    }
}
