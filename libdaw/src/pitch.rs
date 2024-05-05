mod parse;

use crate::parse::{Error, IResult};
use nom::{combinator::all_consuming, Finish};
use std::fmt;
use std::fmt::Debug;
use std::str::FromStr;

/// A relative pitch within an octave, corresponding to the western note names
/// and a standard C major scale.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u8)]
pub enum PitchName {
    C = 0,
    D = 2,
    E = 4,
    F = 5,
    G = 7,
    A = 9,
    B = 11,
}

impl PitchName {
    pub fn name(self) -> char {
        match self {
            PitchName::C => 'C',
            PitchName::D => 'D',
            PitchName::E => 'E',
            PitchName::F => 'F',
            PitchName::G => 'G',
            PitchName::A => 'A',
            PitchName::B => 'B',
        }
    }

    pub fn parse(input: &str) -> IResult<&str, Self> {
        parse::pitch_name(input)
    }
}
impl fmt::Display for PitchName {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", (*self).name())
    }
}

impl FromStr for PitchName {
    type Err = Error<String>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let note = all_consuming(parse::pitch_name)(s)
            .finish()
            .map_err(|e| e.to_owned())?
            .1;
        Ok(note)
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct PitchClass {
    pub name: PitchName,
    pub adjustment: f64,
}

impl PitchClass {
    pub fn parse(input: &str) -> IResult<&str, Self> {
        parse::pitch_class(input)
    }
}

/// Can parse a string like C#4 into its absolute note.
/// Can handle adjustments from this set: #b♭♯𝄳𝄫𝄪𝄲♮,'
/// Can also handle numeric adjustments, expressed in semitones, in square brackets,
/// and ratios of these, along with symbolic ones.
/// B𝄫𝄪###[14/12e8]-12 is a valid (but completely inaudible) absolute note.
impl FromStr for PitchClass {
    type Err = Error<String>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let note = all_consuming(parse::pitch_class)(s)
            .finish()
            .map_err(|e| e.to_owned())?
            .1;
        Ok(note)
    }
}

/// An absolute pitch, with the octave and any adjustments specified.  This lets
/// you get any frequency, subject to the PitchStandard used.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Pitch {
    pub pitch_class: PitchClass,
    pub octave: i8,
}

impl Pitch {
    pub fn parse(input: &str) -> IResult<&str, Self> {
        parse::pitch(input)
    }
}

/// Can parse a string like C#4 into its absolute note.
/// Can handle adjustments from this set: #b♭♯𝄳𝄫𝄪𝄲♮,'
/// Can also handle numeric adjustments, expressed in semitones, in square brackets,
/// and ratios of these, along with symbolic ones.
/// B𝄫𝄪###[14/12e8]-12 is a valid (but completely inaudible) absolute note.
impl FromStr for Pitch {
    type Err = Error<String>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let note = all_consuming(parse::pitch)(s)
            .finish()
            .map_err(|e| e.to_owned())?
            .1;
        Ok(note)
    }
}

pub trait PitchStandard: Debug + Send + Sync {
    /// Resolve a pitch to a frequency.
    fn resolve(&self, pitch: Pitch) -> f64;
}

trait TwelveToneEqualTemperament {
    fn c0() -> f64;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ScientificPitch;

impl TwelveToneEqualTemperament for ScientificPitch {
    fn c0() -> f64 {
        16.0
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct A440;

impl TwelveToneEqualTemperament for A440 {
    fn c0() -> f64 {
        // 440.0 / (2.0.powf(4.0 + 9.0 / 12.0))
        16.351597831287414667
    }
}

impl<T> PitchStandard for T
where
    T: TwelveToneEqualTemperament + Debug + Send + Sync,
{
    fn resolve(&self, pitch: Pitch) -> f64 {
        let exponent_numerator = pitch.octave as f64 * 12.0
            + pitch.pitch_class.name as i8 as f64
            + pitch.pitch_class.adjustment;
        Self::c0() * 2.0f64.powf(exponent_numerator / 12.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    fn round(a: f64, magnitude: f64) -> f64 {
        (a * magnitude).round() / magnitude
    }
    #[test]
    fn a440() {
        assert_eq!(
            round(
                A440.resolve(Pitch {
                    octave: 4,
                    pitch_class: PitchClass {
                        name: PitchName::A,
                        adjustment: 0.0
                    }
                }),
                1.0e10
            ),
            440.0,
        );
    }
    #[test]
    fn scientific_pitch() {
        assert_eq!(
            round(
                ScientificPitch.resolve(Pitch {
                    octave: 4,
                    pitch_class: PitchClass {
                        name: PitchName::C,
                        adjustment: 0.0
                    }
                }),
                1.0e10
            ),
            256.0,
        );
    }
}
