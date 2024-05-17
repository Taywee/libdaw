use super::Pitch;
use crate::{metronome::Beat, notation::Chord, parse::IResult};
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{multispace0, multispace1},
    combinator::{map, opt},
    multi::separated_list1,
    sequence::preceded,
};
use std::sync::{Arc, Mutex};

pub fn chord(input: &str) -> IResult<&str, Chord> {
    let (input, _) = alt((tag("="), tag("chord")))(input)?;
    let (input, _) = multispace0(input)?;
    let (input, _) = tag("(")(input)?;
    let (input, _) = multispace0(input)?;
    let (input, pitches) = separated_list1(
        multispace1,
        map(Pitch::parse, |pitch| Arc::new(Mutex::new(pitch))),
    )(input)?;
    let (input, _) = multispace0(input)?;
    let (input, _) = tag(")")(input)?;
    let (input, length) = opt(preceded(tag(":"), Beat::parse))(input)?;
    let (input, duration) = opt(preceded(tag(":"), Beat::parse))(input)?;
    Ok((
        input,
        Chord {
            pitches,
            length,
            duration,
        },
    ))
}
