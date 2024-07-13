mod parse;

use super::{
    tone_generation_state::ToneGenerationState, Chord, Mode, Note, Overlapped, Rest, Scale,
    Sequence, Set,
};
use crate::{
    metronome::{Beat, Metronome},
    nodes::instrument::Tone,
    parse::IResult,
    pitch::PitchStandard,
};
use nom::{combinator::all_consuming, error::convert_error, Finish as _};
use std::{
    fmt,
    str::FromStr,
    sync::{Arc, Mutex},
};

pub trait ItemValue: fmt::Debug {
    /// Resolve all the section's notes to playable instrument tones.
    fn tones(
        &self,
        _offset: Beat,
        _metronome: &Metronome,
        _pitch_standard: &dyn PitchStandard,
        _state: &ToneGenerationState,
    ) -> Box<dyn Iterator<Item = Tone> + 'static> {
        Box::new(std::iter::empty())
    }

    fn length(&self, _state: &ToneGenerationState) -> Beat {
        Beat::ZERO
    }

    fn update_state(&self, _state: &mut ToneGenerationState) {}

    fn duration(&self, _state: &ToneGenerationState) -> Beat {
        Beat::ZERO
    }
}

#[derive(Clone, Debug)]
pub struct Item {
    pub value: Arc<Mutex<dyn ItemValue>>,
}

impl Item {
    /// Resolve all the section's notes to playable instrument tones.
    /// The offset is the beat offset.
    pub fn tones(
        &self,
        offset: Beat,
        metronome: &Metronome,
        pitch_standard: &dyn PitchStandard,
        state: &ToneGenerationState,
    ) -> Box<dyn Iterator<Item = Tone> + 'static> {
        self.value
            .lock()
            .expect("poisoned")
            .tones(offset, metronome, pitch_standard, state)
    }
    pub fn length(&self, state: &ToneGenerationState) -> Beat {
        self.value.lock().expect("poisoned").length(state)
    }

    pub fn update_state(&self, state: &mut ToneGenerationState) {
        self.value.lock().expect("poisoned").update_state(state)
    }

    pub fn duration(&self, state: &ToneGenerationState) -> Beat {
        self.value.lock().expect("poisoned").duration(state)
    }

    pub fn parse(input: &str) -> IResult<&str, Self> {
        parse::item(input)
    }
}

impl FromStr for Item {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let note = all_consuming(Self::parse)(s)
            .finish()
            .map_err(move |e| convert_error(s, e))?
            .1;
        Ok(note)
    }
}
