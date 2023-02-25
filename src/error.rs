use std::error;
use std::fmt;
use std::io;
use std::num::ParseIntError;
use std::result;

pub type Result<R> = result::Result<R, Error>;

#[derive(Debug)]
pub enum Error {
    IoError(io::Error),
    SerializationError(serde_json::Error),
    IntParseError(ParseIntError),
    TokenParseError(String),
}

use Error::*;

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            IoError(ref err) => err.fmt(f),
            SerializationError(ref err) => err.fmt(f),
            IntParseError(ref err) => err.fmt(f),
            TokenParseError(ref input) => write!(f, "token parsing error on {:?}", input),
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