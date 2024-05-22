use super::Step;
use crate::parse::{numeric_adjustment, octave_shift, IResult};
use nom::{character::complete::i64, combinator::opt};

pub fn step(input: &str) -> IResult<&str, Step> {
    let (input, step) = i64(input)?;
    let (input, numeric_adjustment) = opt(numeric_adjustment)(input)?;
    let (input, octave_shift) = opt(octave_shift)(input)?;
    Ok((
        input,
        Step {
            step,
            octave_shift: octave_shift.unwrap_or_default(),
            adjustment: numeric_adjustment.unwrap_or_default(),
        },
    ))
}
