mod parse;

use super::{
    tone_generation_state::ToneGenerationState, Chord, Element, Mode, Note, Overlapped, Rest,
    Scale, Sequence, Set,
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

/// A container type for the things inside an Item, allowing in-place
/// modification of an Item's value.  We can't use an Arc<Mutex<dyn Element>>
/// directly, otherwise after parsing an Item, we won't be able to find what the
/// actual type is (unless we do something like use Any, but then we still can't
/// get an Arc<Mutex<Note>> from Arc<Mutex<dyn Element>> anyway)).
#[derive(Clone)]
pub enum ItemElement {
    Note(Arc<Mutex<Note>>),
    Chord(Arc<Mutex<Chord>>),
    Rest(Arc<Mutex<Rest>>),
    Overlapped(Arc<Mutex<Overlapped>>),
    Sequence(Arc<Mutex<Sequence>>),
    Scale(Arc<Mutex<Scale>>),
    Mode(Arc<Mutex<Mode>>),
    Set(Arc<Mutex<Set>>),
}

impl fmt::Debug for ItemElement {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ItemElement::Note(note) => fmt::Debug::fmt(&note.lock().expect("poisoned"), f),
            ItemElement::Chord(chord) => fmt::Debug::fmt(&chord.lock().expect("poisoned"), f),
            ItemElement::Rest(rest) => fmt::Debug::fmt(&rest.lock().expect("poisoned"), f),
            ItemElement::Overlapped(overlapped) => {
                fmt::Debug::fmt(&overlapped.lock().expect("poisoned"), f)
            }
            ItemElement::Sequence(sequence) => {
                fmt::Debug::fmt(&sequence.lock().expect("poisoned"), f)
            }
            ItemElement::Scale(scale) => fmt::Debug::fmt(&scale.lock().expect("poisoned"), f),
            ItemElement::Mode(mode) => fmt::Debug::fmt(&mode.lock().expect("poisoned"), f),
            ItemElement::Set(set) => fmt::Debug::fmt(&set.lock().expect("poisoned"), f),
        }
    }
}

impl Element for ItemElement {
    /// Resolve all the section's notes to playable instrument tones.
    /// The offset is the beat offset.
    fn tones(
        &self,
        metronome: &Metronome,
        pitch_standard: &dyn PitchStandard,
        state: &ToneGenerationState,
    ) -> Box<dyn Iterator<Item = Tone> + 'static> {
        self.as_dyn()
            .lock()
            .expect("poisoned")
            .tones(metronome, pitch_standard, state)
    }

    fn length(&self, state: &ToneGenerationState) -> Beat {
        self.as_dyn().lock().expect("poisoned").length(state)
    }

    fn update_state(&self, state: &mut ToneGenerationState) {
        self.as_dyn().lock().expect("poisoned").update_state(state)
    }

    fn duration(&self, state: &ToneGenerationState) -> Beat {
        self.as_dyn().lock().expect("poisoned").duration(state)
    }
}
impl ItemElement {
    pub fn parse(input: &str) -> IResult<&str, Self> {
        parse::item_element(input)
    }
    pub fn as_dyn(&self) -> Arc<Mutex<dyn Element>> {
        match self.clone() {
            ItemElement::Note(note) => return note,
            ItemElement::Chord(chord) => return chord,
            ItemElement::Rest(rest) => return rest,
            ItemElement::Overlapped(overlapped) => return overlapped,
            ItemElement::Sequence(sequence) => return sequence,
            ItemElement::Scale(scale) => return scale,
            ItemElement::Mode(mode) => return mode,
            ItemElement::Set(set) => return set,
        };
    }
}

impl FromStr for ItemElement {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let note = all_consuming(Self::parse)(s)
            .finish()
            .map_err(move |e| convert_error(s, e))?
            .1;
        Ok(note)
    }
}

#[derive(Clone, Debug)]
pub struct Item {
    pub element: ItemElement,
}

impl Element for Item {
    /// Resolve all the section's notes to playable instrument tones.
    /// The offset is the beat offset.
    fn tones(
        &self,
        metronome: &Metronome,
        pitch_standard: &dyn PitchStandard,
        state: &ToneGenerationState,
    ) -> Box<dyn Iterator<Item = Tone> + 'static> {
        self.element.tones(metronome, pitch_standard, state)
    }
    fn length(&self, state: &ToneGenerationState) -> Beat {
        self.element.length(state)
    }

    fn update_state(&self, state: &mut ToneGenerationState) {
        self.element.update_state(state)
    }

    fn duration(&self, state: &ToneGenerationState) -> Beat {
        self.element.duration(state)
    }
}

impl Item {
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
