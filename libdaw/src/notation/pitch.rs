mod parse;

use crate::{
    parse::{Error, IResult},
    pitch::{Pitch as AbsolutePitch, PitchClass},
};
use nom::{combinator::all_consuming, Finish as _};
use std::{
    str::FromStr,
    sync::{Arc, Mutex},
};

use super::resolve_state::ResolveState;

/// A notation-specific pitch specification, which may be relative or even a
/// scale-mode representition (TODO)
#[derive(Debug, Clone)]
pub struct Pitch {
    pub pitch_class: Arc<Mutex<PitchClass>>,
    pub octave: Option<i8>,
    pub octave_shift: i8,
}

impl Pitch {
    pub fn parse(input: &str) -> IResult<&str, Self> {
        parse::pitch(input)
    }
    /// Resolve to an absolute pitch
    pub fn absolute(&self, state: &ResolveState) -> AbsolutePitch {
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
    type Err = Error<String>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let pitch = all_consuming(Self::parse)(s)
            .finish()
            .map_err(|e| e.to_owned())?
            .1;
        Ok(pitch)
    }
}
