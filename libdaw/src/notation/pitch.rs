mod parse;

use crate::{
    parse::IResult,
    pitch::{Pitch as AbsolutePitch, PitchClass},
};
use nom::{combinator::all_consuming, error::convert_error, Finish as _};
use std::{
    fmt,
    str::FromStr,
    sync::{Arc, Mutex},
};

use super::tone_generation_state::ToneGenerationState;

/// A notation-specific pitch specification, which may be absolute or relative.
#[derive(Clone)]
pub struct Pitch {
    pub pitch_class: Arc<Mutex<PitchClass>>,
    pub octave: Option<i8>,
    pub octave_shift: i8,
}

impl fmt::Debug for Pitch {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let pitch_class = self.pitch_class.lock().expect("poisoned");
        let pitch_class = &*pitch_class;
        f.debug_struct("Pitch")
            .field("pitch_class", pitch_class)
            .field("octave", &self.octave)
            .field("octave_shift", &self.octave_shift)
            .finish()
    }
}

impl Pitch {
    pub fn parse(input: &str) -> IResult<&str, Self> {
        parse::pitch(input)
    }
    /// Resolve to an absolute pitch
    pub(super) fn absolute(&self, state: &ToneGenerationState) -> AbsolutePitch {
        let unshifted_octave = self.octave.unwrap_or_else(|| {
            let a = state.pitch.pitch_class.lock().expect("poisoned");
            let b = self.pitch_class.lock().expect("poisoned");
            let relative_shift = a.name.octave_shift_for_closest(b.name);

            state.pitch.octave + relative_shift
        });
        AbsolutePitch {
            pitch_class: self.pitch_class.clone(),
            octave: unshifted_octave + self.octave_shift,
        }
    }
}
impl FromStr for Pitch {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let pitch = all_consuming(Self::parse)(s)
            .finish()
            .map_err(move |e| convert_error(s, e))?
            .1;
        Ok(pitch)
    }
}
