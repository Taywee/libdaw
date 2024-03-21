pub mod error;
pub mod notation;
pub mod pitch;

pub use error::Error;
use nom::{bytes::complete::tag, combinator::opt, number::complete::double};

use crate::metronome::Beat;

pub type IResult<I, O> = nom::IResult<I, O, Error<I>>;

fn denominator(input: &str) -> IResult<&str, f64> {
    let (input, _) = tag("/")(input)?;
    let (input, denominator) = double(input)?;
    Ok((input, denominator))
}

/// A floating point number, optionally divided by another floating point number.
fn number(input: &str) -> IResult<&str, f64> {
    let (input, numerator) = double(input)?;
    let (input, denominator) = opt(denominator)(input)?;
    let number = match denominator {
        Some(denominator) => numerator / denominator,
        None => numerator,
    };
    Ok((input, number))
}

/// Parse a number using the `number` parser and turn it into a beat.
fn beat(input: &str) -> IResult<&str, Beat> {
    let (input, number) = number(input)?;
    let beat = Beat::new(number).ok_or_else(move || nom::Err::Error(Error::IllegalBeat(number)))?;
    Ok((input, beat))
}
