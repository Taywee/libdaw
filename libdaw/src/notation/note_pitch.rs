mod parse;

use super::{tone_generation_state::ToneGenerationState, Pitch, Step};
use crate::{parse::IResult, pitch::Pitch as AbsolutePitch};
use nom::{combinator::all_consuming, error::convert_error, Finish as _};
use std::{
    fmt,
    str::FromStr,
    sync::{Arc, Mutex},
};

#[derive(Clone)]
pub enum NotePitch {
    Pitch(Arc<Mutex<Pitch>>),
    Step(Arc<Mutex<Step>>),
}

impl fmt::Debug for NotePitch {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            NotePitch::Pitch(pitch) => fmt::Debug::fmt(&pitch.lock().expect("poisoned"), f),
            NotePitch::Step(step) => fmt::Debug::fmt(&step.lock().expect("poisoned"), f),
        }
    }
}

impl NotePitch {
    pub fn parse(input: &str) -> IResult<&str, Self> {
        parse::note_pitch(input)
    }
    /// Resolve to an absolute pitch
    pub(super) fn absolute(&self, state: &ToneGenerationState) -> AbsolutePitch {
        match self {
            NotePitch::Pitch(pitch) => pitch.lock().expect("poisoned").absolute(state),
            NotePitch::Step(step) => step.lock().expect("poisoned").absolute(state),
        }
    }
    pub(super) fn update_state(&self, state: &mut ToneGenerationState) {
        let pitch = self.absolute(state);
        state.pitch = pitch;
        if let Self::Step(step) = self {
            step.lock().expect("poisoned").update_state(state);
        }
    }
}

impl FromStr for NotePitch {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let note = all_consuming(Self::parse)(s)
            .finish()
            .map_err(move |e| convert_error(s, e))?
            .1;
        Ok(note)
    }
}
