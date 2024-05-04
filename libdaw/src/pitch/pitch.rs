use crate::parse::pitch as parse;
use crate::parse::Error;
use nom::{combinator::all_consuming, Finish};
use std::fmt;
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
