use crate::{metronome::Beat, notation::absolute::Chord, parse::IResult, pitch::Pitch};
use nom::{
    bytes::complete::tag,
    character::complete::{multispace0, multispace1},
    combinator::opt,
    multi::separated_list1,
    sequence::preceded,
};

pub fn chord(input: &str) -> IResult<&str, Chord> {
    let (input, _) = tag("{")(input)?;
    let (input, _) = multispace0(input)?;
    let (input, pitches) = separated_list1(multispace1, Pitch::parse)(input)?;
    let (input, _) = multispace0(input)?;
    let (input, _) = tag("}")(input)?;
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
