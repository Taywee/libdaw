use std::sync::{Arc, Mutex};

use crate::parse::{numeric_adjustment, IResult};
use crate::pitch::{Pitch, PitchClass, PitchName};
use nom::character::complete::i8;
use nom::{character::complete::one_of, combinator::opt, multi::fold_many0};

pub fn pitch_name(input: &str) -> IResult<&str, PitchName> {
    let (input, note) = one_of("cdefgabCDEFGAB")(input)?;
    let note = match note {
        'C' | 'c' => PitchName::C,
        'D' | 'd' => PitchName::D,
        'E' | 'e' => PitchName::E,
        'F' | 'f' => PitchName::F,
        'G' | 'g' => PitchName::G,
        'A' | 'a' => PitchName::A,
        'B' | 'b' => PitchName::B,
        _ => unreachable!(),
    };
    Ok((input, note))
}
fn adjustment_symbol(input: &str) -> IResult<&str, f64> {
    let (input, symbol) = one_of("#b♭♯𝄳𝄫𝄪𝄲♮")(input)?;
    let adjustment = match symbol {
        '𝄫' => -2.0,
        'b' => -1.0,
        '♭' => -1.0,
        '𝄳' => -0.5,
        '♮' => 0.0,
        '𝄲' => 0.5,
        '#' => 1.0,
        '♯' => 1.0,
        '𝄪' => 2.0,
        _ => unreachable!(),
    };
    Ok((input, adjustment))
}

fn symbol_adjustments(input: &str) -> IResult<&str, f64> {
    fold_many0(adjustment_symbol, || 0.0f64, |acc, item| acc + item)(input)
}

fn adjustment(input: &str) -> IResult<&str, f64> {
    let (input, symbolic_adjustment) = symbol_adjustments(input)?;
    let (input, numeric_adjustment) = opt(numeric_adjustment)(input)?;
    Ok((
        input,
        symbolic_adjustment + numeric_adjustment.unwrap_or(0.0),
    ))
}

pub fn pitch_class(input: &str) -> IResult<&str, PitchClass> {
    let (input, note) = pitch_name(input)?;
    let (input, adjustment) = adjustment(input)?;
    Ok((
        input,
        PitchClass {
            name: note,
            adjustment,
        },
    ))
}

pub fn pitch(input: &str) -> IResult<&str, Pitch> {
    let (input, pitch_class) = pitch_class(input)?;
    let (input, octave) = i8(input)?;
    Ok((
        input,
        Pitch {
            pitch_class: Arc::new(Mutex::new(pitch_class)),
            octave,
        },
    ))
}
