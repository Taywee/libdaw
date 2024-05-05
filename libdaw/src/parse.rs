//! Common parsers and types for parsers.

pub mod error;

pub use error::Error;
use nom::{bytes::complete::tag, combinator::opt, number::complete::double};

pub type IResult<I, O> = nom::IResult<I, O, Error<I>>;

pub fn denominator(input: &str) -> IResult<&str, f64> {
    let (input, _) = tag("/")(input)?;
    let (input, denominator) = double(input)?;
    Ok((input, denominator))
}

/// A floating point number, optionally divided by another floating point number.
pub fn number(input: &str) -> IResult<&str, f64> {
    let (input, numerator) = double(input)?;
    let (input, denominator) = opt(denominator)(input)?;
    let number = match denominator {
        Some(denominator) => numerator / denominator,
        None => numerator,
    };
    Ok((input, number))
}
