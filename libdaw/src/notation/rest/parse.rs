use crate::{metronome::Beat, notation::Rest, parse::IResult};
use nom::{bytes::complete::tag, combinator::opt, sequence::preceded};

pub fn rest(input: &str) -> IResult<&str, Rest> {
    let (input, _) = tag("r")(input)?;
    let (input, length) = opt(preceded(tag(","), Beat::parse))(input)?;
    Ok((input, Rest { length }))
}
