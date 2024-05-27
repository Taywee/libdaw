use crate::{metronome::Beat, notation::Rest, parse::IResult};
use nom::{character::complete::char, combinator::opt, sequence::preceded};

pub fn rest(input: &str) -> IResult<&str, Rest> {
    let (input, _) = char('r')(input)?;
    let (input, length) = opt(preceded(char(','), Beat::parse))(input)?;
    Ok((input, Rest { length }))
}
