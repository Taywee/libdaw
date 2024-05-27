mod parse;

use crate::parse::IResult;
use nom::error::convert_error;
use nom::{combinator::all_consuming, Finish};
use std::fmt;
use std::str::FromStr;
use std::sync::{Arc, Mutex};

/// A relative pitch within an octave, corresponding to the western note names
/// and a standard C major scale.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum PitchName {
    C,
    D,
    E,
    F,
    G,
    A,
    B,
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

    pub fn semitone_shift(self) -> u8 {
        match self {
            Self::C => 0,
            Self::D => 2,
            Self::E => 4,
            Self::F => 5,
            Self::G => 7,
            Self::A => 9,
            Self::B => 11,
        }
    }

    pub fn index(self) -> u8 {
        match self {
            Self::C => 0,
            Self::D => 1,
            Self::E => 2,
            Self::F => 3,
            Self::G => 4,
            Self::A => 5,
            Self::B => 6,
        }
    }

    /// Gives a relative octave shift for two names that will keep them as close
    /// as possible in Pitch distance, disregarding adjustments.
    pub fn octave_shift_for_closest(self, other: Self) -> i8 {
        let a = self.index();
        let b = other.index();
        if a + 3 < b {
            -1
        } else if b + 3 < a {
            1
        } else {
            0
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
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let note = all_consuming(Self::parse)(s)
            .finish()
            .map_err(move |e| convert_error(s, e))?
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
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let note = all_consuming(Self::parse)(s)
            .finish()
            .map_err(move |e| convert_error(s, e))?
            .1;
        Ok(note)
    }
}

/// An absolute pitch, with the octave and any adjustments specified.  This lets
/// you get any frequency, subject to the PitchStandard used.
#[derive(Clone)]
pub struct Pitch {
    pub pitch_class: Arc<Mutex<PitchClass>>,
    pub octave: i8,
}

impl fmt::Debug for Pitch {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let pitch_class = self.pitch_class.lock().expect("poisoned");
        let pitch_class = &*pitch_class;
        f.debug_struct("Pitch")
            .field("pitch_class", pitch_class)
            .field("octave", &self.octave)
            .finish()
    }
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
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let note = all_consuming(Self::parse)(s)
            .finish()
            .map_err(move |e| convert_error(s, e))?
            .1;
        Ok(note)
    }
}

pub trait PitchStandard: fmt::Debug + Send + Sync {
    /// Resolve a pitch to a frequency.
    fn resolve(&self, pitch: &Pitch) -> f64;
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
    T: TwelveToneEqualTemperament + fmt::Debug + Send + Sync,
{
    fn resolve(&self, pitch: &Pitch) -> f64 {
        let pitch_class = pitch.pitch_class.lock().expect("poisoned");
        let exponent_numerator = pitch.octave as f64 * 12.0
            + pitch_class.name.semitone_shift() as i8 as f64
            + pitch_class.adjustment;
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
                A440.resolve(&Pitch {
                    octave: 4,
                    pitch_class: Arc::new(Mutex::new(PitchClass {
                        name: PitchName::A,
                        adjustment: 0.0
                    })),
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
                ScientificPitch.resolve(&Pitch {
                    octave: 4,
                    pitch_class: Arc::new(Mutex::new(PitchClass {
                        name: PitchName::C,
                        adjustment: 0.0
                    })),
                }),
                1.0e10
            ),
            256.0,
        );
    }
}
