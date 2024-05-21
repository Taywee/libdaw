use super::{NotePitch, Pitch, Step};
use crate::{metronome::Beat, notation::Note, parse::IResult};
use nom::{
    branch::alt,
    bytes::complete::tag,
    combinator::{map, opt},
    sequence::preceded,
};
use std::sync::{Arc, Mutex};

pub fn note_pitch(input: &str) -> IResult<&str, NotePitch> {
    alt((
        map(Pitch::parse, |pitch| {
            NotePitch::Pitch(Arc::new(Mutex::new(pitch)))
        }),
        map(Step::parse, |pitch| {
            NotePitch::Step(Arc::new(Mutex::new(pitch)))
        }),
    ))(input)
}

pub fn note(input: &str) -> IResult<&str, Note> {
    let (input, pitch) = note_pitch(input)?;
    let (input, length) = opt(preceded(tag(","), Beat::parse))(input)?;
    let (input, duration) = opt(preceded(tag(","), Beat::parse))(input)?;
    Ok((
        input,
        Note {
            pitch,
            length,
            duration,
        },
    ))
}
