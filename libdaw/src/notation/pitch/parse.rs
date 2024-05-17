use super::Pitch;
use crate::{parse::IResult, pitch::PitchClass};
use nom::{
    character::complete::{digit1, i8, one_of},
    combinator::opt,
};
use std::sync::{Arc, Mutex};

/// Parse the octave, which must be digits without a sign.
pub fn octave(input: &str) -> IResult<&str, i8> {
    // Ensure that it starts with digits.
    let _ = digit1(input)?;
    i8(input)
}

/// Parse the octave shift, which must be digits with a sign.
pub fn octave_shift(input: &str) -> IResult<&str, i8> {
    // Ensure that it starts with a sign.
    let _ = one_of("+-")(input)?;
    i8(input)
}

pub fn pitch(input: &str) -> IResult<&str, Pitch> {
    let (input, pitch_class) = PitchClass::parse(input)?;
    let (input, octave) = opt(octave)(input)?;
    let (input, octave_shift) = opt(octave_shift)(input)?;
    Ok((
        input,
        Pitch {
            pitch_class: Arc::new(Mutex::new(pitch_class)),
            octave,
            octave_shift: octave_shift.unwrap_or(0),
        },
    ))
}
