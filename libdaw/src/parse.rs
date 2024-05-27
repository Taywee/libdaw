//! Common parsers and types for parsers.

use nom::{
    branch::alt,
    character::complete::char,
    combinator::{cut, map_res, opt},
    error::VerboseError,
    multi::many1_count,
    number::complete::double,
};

pub type IResult<I, O> = nom::IResult<I, O, VerboseError<I>>;

pub fn denominator(input: &str) -> IResult<&str, f64> {
    let (input, _) = char('/')(input)?;
    let (input, denominator) = cut(double)(input)?;
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

fn plus_signs(input: &str) -> IResult<&str, i8> {
    map_res(many1_count(char('+')), i8::try_from)(input)
}
fn minus_signs(input: &str) -> IResult<&str, i8> {
    map_res(many1_count(char('-')), |count| {
        let count: i16 = count.try_into()?;
        (-count).try_into()
    })(input)
}

/// Parse the octave shift, which must be a series of plus signes or minus
/// signs.
pub fn octave_shift(input: &str) -> IResult<&str, i8> {
    alt((plus_signs, minus_signs))(input)
}

pub fn numeric_adjustment(input: &str) -> IResult<&str, f64> {
    let (input, _) = char('[')(input)?;
    let (input, adjustment) = number(input)?;
    let (input, _) = char(']')(input)?;
    Ok((input, adjustment))
}
