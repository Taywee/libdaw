use super::{Duration, NotePitch};
use crate::{metronome::Beat, notation::Note, parse::IResult};
use nom::{character::complete::char, combinator::opt, sequence::preceded};

pub fn note(input: &str) -> IResult<&str, Note> {
    let (input, pitch) = NotePitch::parse(input)?;
    let (input, length) = opt(preceded(char(','), Beat::parse))(input)?;
    let (input, duration) = opt(preceded(char(','), Duration::parse))(input)?;
    Ok((
        input,
        Note {
            pitch,
            length,
            duration,
        },
    ))
}
