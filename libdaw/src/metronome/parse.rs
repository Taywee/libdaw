use crate::parse::{Error, IResult};
use crate::{metronome::Beat, parse::number};

/// Parse a number using the `number` parser and turn it into a beat.
pub fn beat(input: &str) -> IResult<&str, Beat> {
    let (input, number) = number(input)?;
    let beat = Beat::new(number).map_err(move |e| nom::Err::Error(Error::IllegalBeat(e)))?;
    Ok((input, beat))
}
