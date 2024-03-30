use nom::error::ErrorKind;
use std::{fmt, num::ParseIntError};

#[derive(Debug)]
pub enum Error<I> {
    Nom(nom::error::Error<I>),
    ParseInt(ParseIntError),
    IllegalBeat(f64),
}

impl Error<&str> {
    pub fn to_owned(self) -> Error<String> {
        match self {
            Error::Nom(nom::error::Error { input, code }) => Error::Nom(nom::error::Error {
                input: input.to_owned(),
                code,
            }),
            Error::ParseInt(i) => Error::ParseInt(i),
            Error::IllegalBeat(f) => Error::IllegalBeat(f),
        }
    }
}

impl<I> From<ParseIntError> for Error<I> {
    fn from(v: ParseIntError) -> Self {
        Self::ParseInt(v)
    }
}

impl<I> fmt::Display for Error<I>
where
    I: fmt::Debug,
    I: fmt::Display,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::Nom(e) => write!(f, "parse error: {e}"),
            Error::ParseInt(e) => write!(f, "integer parse error: {e}"),
            Error::IllegalBeat(e) => write!(f, "illegal beat: {e}"),
        }
    }
}

impl<I> std::error::Error for Error<I> where I: fmt::Debug + fmt::Display {}

impl<I> From<nom::error::Error<I>> for Error<I> {
    fn from(v: nom::error::Error<I>) -> Self {
        Self::Nom(v)
    }
}

impl<I> nom::error::ParseError<I> for Error<I> {
    fn from_error_kind(input: I, kind: ErrorKind) -> Self {
        Self::Nom(nom::error::Error::new(input, kind))
    }

    fn append(_: I, _: ErrorKind, other: Self) -> Self {
        other
    }
}
