use super::{Chord, Duration, NotePitch, StateMember};
use crate::{metronome::Beat, parse::IResult};
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{char, multispace0, multispace1},
    combinator::{cut, opt},
    multi::separated_list0,
    sequence::preceded,
};

pub fn chord(input: &str) -> IResult<&str, Chord> {
    let (input, _) = alt((tag("="), tag("chord")))(input)?;
    let (input, _) = multispace0(input)?;
    let (input, state_member) = opt(StateMember::parse)(input)?;
    let (input, _) = multispace0(input)?;
    let (input, _) = cut(char('('))(input)?;
    let (input, _) = multispace0(input)?;
    let (input, pitches) = cut(separated_list0(multispace1, NotePitch::parse))(input)?;
    let (input, _) = multispace0(input)?;
    let (input, _) = cut(char(')'))(input)?;
    let (input, length) = opt(preceded(char(','), Beat::parse))(input)?;
    let (input, duration) = opt(preceded(char(','), Duration::parse))(input)?;
    Ok((
        input,
        Chord {
            pitches,
            length,
            duration,
            state_member,
        },
    ))
}
