use ordered_float::FloatIsNan;
use std::fmt;

#[derive(Debug)]
pub enum Error {
    IllegalBeat(f64),
    IllegalBeatsPerMinute(f64),
    FloatIsNan,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::IllegalBeat(beat) => write!(f, "illegal beat value: {beat}"),
            Error::IllegalBeatsPerMinute(beats_per_minute) => {
                write!(f, "illegal beats_per_minute value: {beats_per_minute}")
            }
            Error::FloatIsNan => write!(f, "calculation produced a NaN value"),
        }
    }
}

impl std::error::Error for Error {}

impl From<FloatIsNan> for Error {
    fn from(_: FloatIsNan) -> Self {
        Error::FloatIsNan
    }
}
