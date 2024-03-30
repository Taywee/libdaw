use crate::parse::{Error, IResult};
use crate::pitch::{Pitch, PitchClass};
use nom::{
    bytes::complete::tag,
    character::complete::{digit1, one_of},
    combinator::{opt, recognize},
    multi::fold_many0,
    sequence::preceded,
};

pub fn pitch_class(input: &str) -> IResult<&str, PitchClass> {
    let (input, note) = one_of("cdefgabCDEFGAB")(input)?;
    let note = match note {
        'C' | 'c' => PitchClass::C,
        'D' | 'd' => PitchClass::D,
        'E' | 'e' => PitchClass::E,
        'F' | 'f' => PitchClass::F,
        'G' | 'g' => PitchClass::G,
        'A' | 'a' => PitchClass::A,
        'B' | 'b' => PitchClass::B,
        _ => unreachable!(),
    };
    Ok((input, note))
}
fn adjustment_symbol(input: &str) -> IResult<&str, f64> {
    let (input, symbol) = one_of("#b♭♯𝄳𝄫𝄪𝄲♮,'")(input)?;
    let adjustment = match symbol {
        '𝄫' => -2.0,
        'b' => -1.0,
        'f' => -1.0,
        ',' => -1.0,
        '♭' => -1.0,
        '𝄳' => -0.5,
        '♮' => 0.0,
        '𝄲' => 0.5,
        '#' => 1.0,
        's' => 1.0,
        '♯' => 1.0,
        '\'' => 1.0,
        '𝄪' => 2.0,
        _ => unreachable!(),
    };
    Ok((input, adjustment))
}

fn symbol_adjustments(input: &str) -> IResult<&str, f64> {
    fold_many0(adjustment_symbol, || 0.0f64, |acc, item| acc + item)(input)
}

fn numeric_adjustment(input: &str) -> IResult<&str, f64> {
    let (input, _) = tag("[")(input)?;
    let (input, adjustment) = crate::parse::number(input)?;
    let (input, _) = tag("]")(input)?;
    Ok((input, adjustment))
}

fn adjustment(input: &str) -> IResult<&str, f64> {
    let (input, symbolic_adjustment) = symbol_adjustments(input)?;
    let (input, numeric_adjustment) = opt(numeric_adjustment)(input)?;
    Ok((
        input,
        symbolic_adjustment + numeric_adjustment.unwrap_or(0.0),
    ))
}

fn octave(input: &str) -> IResult<&str, i8> {
    let (input, octave_str) = recognize(preceded(opt(tag("-")), digit1))(input)?;
    let octave = octave_str
        .parse()
        .map_err(|e| nom::Err::Error(Error::from(e)))?;
    Ok((input, octave))
}

pub fn pitch(input: &str) -> IResult<&str, Pitch> {
    let (input, note) = pitch_class(input)?;
    let (input, adjustment) = adjustment(input)?;
    let (input, octave) = octave(input)?;
    Ok((
        input,
        Pitch {
            octave,
            class: note,
            adjustment,
        },
    ))
}
