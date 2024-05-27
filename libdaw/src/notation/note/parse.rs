use super::{Duration, NotePitch};
use crate::{metronome::Beat, notation::Note, parse::IResult};
use nom::{bytes::complete::tag, combinator::opt, sequence::preceded};

pub fn note(input: &str) -> IResult<&str, Note> {
    let (input, pitch) = NotePitch::parse(input)?;
    let (input, length) = opt(preceded(tag(","), Beat::parse))(input)?;
    let (input, duration) = opt(preceded(tag(","), Duration::parse))(input)?;
    Ok((
        input,
        Note {
            pitch,
            length,
            duration,
        },
    ))
}
