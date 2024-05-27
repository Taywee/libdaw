use super::{NotePitch, Pitch, Step};
use crate::parse::IResult;
use nom::{branch::alt, combinator::map};
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
