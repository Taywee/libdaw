mod parse;

use std::str::FromStr;

use nom::{combinator::all_consuming, error::convert_error, Finish as _};

use crate::{metronome::Beat, parse::IResult};

#[derive(Clone, Copy, Debug)]
pub enum Duration {
    AddLength(Beat),
    SubtractLength(Beat),
    MultiplyLength(f64),
    Constant(Beat),
}

impl Duration {
    pub fn parse(input: &str) -> IResult<&str, Self> {
        parse::duration(input)
    }

    /// Resolve the duration given the length
    pub fn resolve(self, length: Beat) -> Beat {
        match self {
            Duration::AddLength(value) => length + value,
            Duration::SubtractLength(value) => length - value,
            Duration::MultiplyLength(value) => length * value,
            Duration::Constant(value) => value,
        }
    }
}

impl Default for Duration {
    fn default() -> Self {
        Duration::AddLength(Beat::ZERO)
    }
}

impl FromStr for Duration {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let duration = all_consuming(Self::parse)(s)
            .finish()
            .map_err(move |e| convert_error(s, e))?
            .1;
        Ok(duration)
    }
}
