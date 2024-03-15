use std::{fmt, num::ParseIntError};

use nom::{
    bytes::complete::tag,
    character::complete::{digit1, one_of},
    combinator::{opt, recognize},
    error::{ErrorKind, ParseError},
    multi::{fold_many0, fold_many1, many1},
    number::complete::double,
    sequence::preceded,
};

use super::{Pitch, PitchClass};

#[derive(Debug)]
pub enum Error<I> {
    Nom(nom::error::Error<I>),
    ParseInt(ParseIntError),
}

impl Error<&str> {
    pub fn to_owned(self) -> Error<String> {
        match self {
            Error::Nom(nom::error::Error { input, code }) => Error::Nom(nom::error::Error {
                input: input.to_owned(),
                code,
            }),
            Error::ParseInt(i) => Error::ParseInt(i),
        }
    }
}

impl<I> From<ParseIntError> for Error<I> {
    fn from(v: ParseIntError) -> Self {
        Self::ParseInt(v)
    }
}

impl<I> fmt::Display for Error<I>
where
    I: fmt::Debug,
    I: fmt::Display,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::Nom(e) => write!(f, "parse error: {e}"),
            Error::ParseInt(e) => write!(f, "integer parse error: {e}"),
        }
    }
}

impl<I> std::error::Error for Error<I> where I: fmt::Debug + fmt::Display {}

impl<I> From<nom::error::Error<I>> for Error<I> {
    fn from(v: nom::error::Error<I>) -> Self {
        Self::Nom(v)
    }
}

impl<I> ParseError<I> for Error<I> {
    fn from_error_kind(input: I, kind: ErrorKind) -> Self {
        Self::Nom(nom::error::Error::new(input, kind))
    }

    fn append(_: I, _: ErrorKind, other: Self) -> Self {
        other
    }
}

pub type IResult<I, O> = nom::IResult<I, O, Error<I>>;

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

fn denominator(input: &str) -> IResult<&str, f64> {
    let (input, _) = tag("/")(input)?;
    let (input, denominator) = double(input)?;
    Ok((input, denominator))
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
    let (input, numerator) = double(input)?;
    let (input, denominator) = opt(denominator)(input)?;
    let (input, _) = tag("]")(input)?;
    let adjustment = match denominator {
        Some(denominator) => numerator / denominator,
        None => numerator,
    };
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
