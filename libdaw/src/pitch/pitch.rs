mod parse;

use std::{error::Error, fmt, str::FromStr};

use nom::{combinator::all_consuming, Finish};

/// A relative pitch within an octave, corresponding to the western note names
/// and a standard C major scale.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u8)]
pub enum PitchClass {
    C = 0,
    D = 2,
    E = 4,
    F = 5,
    G = 7,
    A = 9,
    B = 11,
}

impl FromStr for PitchClass {
    type Err = parse::Error<String>;

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
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct Pitch {
    pub octave: i8,
    pub class: PitchClass,
    pub adjustment: f64,
}

/// Can parse a string like C#4 into its absolute note.
/// Can handle adjustments from this set: #b♭♯𝄳𝄫𝄪𝄲♮,'
/// Can also handle numeric adjustments, expressed in semitones, in square brackets,
/// and ratios of these, along with symbolic ones.
/// B𝄫𝄪###[14/12e8]-12 is a valid (but completely inaudible) absolute note.
impl FromStr for Pitch {
    type Err = parse::Error<String>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let note = all_consuming(parse::pitch)(s)
            .finish()
            .map_err(|e| e.to_owned())?
            .1;
        Ok(note)
    }
}
