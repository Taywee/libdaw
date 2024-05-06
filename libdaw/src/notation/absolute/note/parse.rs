use crate::{metronome::Beat, notation::absolute::Note, parse::IResult, pitch::Pitch};
use nom::{bytes::complete::tag, combinator::opt, sequence::preceded};
use std::sync::{Arc, Mutex};

pub fn note(input: &str) -> IResult<&str, Note> {
    let (input, pitch) = Pitch::parse(input)?;
    let (input, length) = opt(preceded(tag(":"), Beat::parse))(input)?;
    let (input, duration) = opt(preceded(tag(":"), Beat::parse))(input)?;
    Ok((
        input,
        Note {
            pitch: Arc::new(Mutex::new(pitch)),
            length,
            duration,
        },
    ))
}
