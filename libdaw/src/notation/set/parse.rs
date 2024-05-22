use super::{NotePitch, Set};
use crate::{metronome::Beat, parse::IResult};
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{multispace0, multispace1},
    combinator::cut,
    multi::separated_list1,
};

fn pitch(input: &str) -> IResult<&str, Set> {
    let (input, _) = tag("pitch")(input)?;
    let (input, _) = multispace0(input)?;
    let (input, _) = cut(tag(":"))(input)?;
    let (input, _) = multispace0(input)?;
    let (input, pitch) = cut(NotePitch::parse)(input)?;
    Ok((
        input,
        Set {
            pitch: Some(pitch),
            ..Default::default()
        },
    ))
}
fn length(input: &str) -> IResult<&str, Set> {
    let (input, _) = tag("length")(input)?;
    let (input, _) = multispace0(input)?;
    let (input, _) = cut(tag(":"))(input)?;
    let (input, _) = multispace0(input)?;
    let (input, length) = cut(Beat::parse)(input)?;
    Ok((
        input,
        Set {
            length: Some(length),
            ..Default::default()
        },
    ))
}

pub fn set(input: &str) -> IResult<&str, Set> {
    let (input, _) = alt((tag(":"), tag("set")))(input)?;
    let (input, _) = multispace0(input)?;
    let (input, _) = cut(tag("("))(input)?;
    let (input, _) = multispace0(input)?;
    let (input, sets) = cut(separated_list1(multispace1, alt((pitch, length))))(input)?;

    // Prefer later settings to earlier
    let set = sets.into_iter().reduce(|b, a| (b | a)).unwrap();
    let (input, _) = multispace0(input)?;
    let (input, _) = cut(tag(")"))(input)?;
    Ok((input, set))
}
