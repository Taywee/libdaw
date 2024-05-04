mod pitch;

pub use pitch::{Pitch, PitchClass, PitchName};

use std::fmt::Debug;

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
