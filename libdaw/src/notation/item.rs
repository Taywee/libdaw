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

/// A container type for the things inside an Item, allowing in-place
/// modification of an Item's value.
#[derive(Clone)]
pub enum ItemValue {
    Note(Arc<Mutex<Note>>),
    Chord(Arc<Mutex<Chord>>),
    Rest(Arc<Mutex<Rest>>),
    Overlapped(Arc<Mutex<Overlapped>>),
    Sequence(Arc<Mutex<Sequence>>),
    Scale(Arc<Mutex<Scale>>),
    Mode(Arc<Mutex<Mode>>),
    Set(Arc<Mutex<Set>>),
}

impl fmt::Debug for ItemValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ItemValue::Note(note) => fmt::Debug::fmt(&note.lock().expect("poisoned"), f),
            ItemValue::Chord(chord) => fmt::Debug::fmt(&chord.lock().expect("poisoned"), f),
            ItemValue::Rest(rest) => fmt::Debug::fmt(&rest.lock().expect("poisoned"), f),
            ItemValue::Overlapped(overlapped) => {
                fmt::Debug::fmt(&overlapped.lock().expect("poisoned"), f)
            }
            ItemValue::Sequence(sequence) => {
                fmt::Debug::fmt(&sequence.lock().expect("poisoned"), f)
            }
            ItemValue::Scale(scale) => fmt::Debug::fmt(&scale.lock().expect("poisoned"), f),
            ItemValue::Mode(mode) => fmt::Debug::fmt(&mode.lock().expect("poisoned"), f),
            ItemValue::Set(set) => fmt::Debug::fmt(&set.lock().expect("poisoned"), f),
        }
    }
}

impl ItemValue {
    /// Resolve all the section's notes to playable instrument tones.
    /// The offset is the beat offset.
    pub(super) fn inner_tones<S>(
        &self,
        offset: Beat,
        metronome: &Metronome,
        pitch_standard: &S,
        state: &ToneGenerationState,
    ) -> Box<dyn Iterator<Item = Tone> + 'static>
    where
        S: PitchStandard + ?Sized,
    {
        match self {
            ItemValue::Note(note) => Box::new(std::iter::once(
                note.lock()
                    .expect("poisoned")
                    .inner_tone(offset, metronome, pitch_standard, state),
            )),
            ItemValue::Chord(chord) => Box::new(chord.lock().expect("poisoned").inner_tones(
                offset,
                metronome,
                pitch_standard,
                state,
            )),
            ItemValue::Overlapped(overlapped) => {
                Box::new(overlapped.lock().expect("poisoned").inner_tones(
                    offset,
                    metronome,
                    pitch_standard,
                    state.clone(),
                ))
            }
            ItemValue::Sequence(sequence) => {
                Box::new(sequence.lock().expect("poisoned").inner_tones(
                    offset,
                    metronome,
                    pitch_standard,
                    state.clone(),
                ))
            }
            ItemValue::Scale(_) | ItemValue::Mode(_) | ItemValue::Rest(_) | ItemValue::Set(_) => {
                Box::new(std::iter::empty())
            }
        }
    }
    pub fn tones<S>(
        &self,
        offset: Beat,
        metronome: &Metronome,
        pitch_standard: &S,
    ) -> Box<dyn Iterator<Item = Tone> + 'static>
    where
        S: PitchStandard + ?Sized,
    {
        self.inner_tones(offset, metronome, pitch_standard, &Default::default())
    }

    pub(super) fn inner_length(&self, state: &ToneGenerationState) -> Beat {
        match self {
            ItemValue::Note(note) => note.lock().expect("poisoned").inner_length(state),
            ItemValue::Chord(chord) => chord.lock().expect("poisoned").inner_length(state),
            ItemValue::Rest(rest) => rest.lock().expect("poisoned").inner_length(state),
            ItemValue::Overlapped(overlapped) => {
                overlapped.lock().expect("poisoned").inner_length(state)
            }
            ItemValue::Sequence(sequence) => sequence
                .lock()
                .expect("poisoned")
                .inner_length(state.clone()),
            ItemValue::Scale(_) | ItemValue::Mode(_) | ItemValue::Set(_) => Beat::ZERO,
        }
    }

    pub(super) fn update_state(&self, state: &mut ToneGenerationState) {
        match self {
            ItemValue::Note(note) => note.lock().expect("poisoned").update_state(state),
            ItemValue::Chord(chord) => chord.lock().expect("poisoned").update_state(state),
            ItemValue::Rest(rest) => rest.lock().expect("poisoned").update_state(state),
            ItemValue::Scale(scale) => scale.lock().expect("poisoned").update_state(state),
            ItemValue::Mode(mode) => mode.lock().expect("poisoned").update_state(state),
            ItemValue::Set(set) => set.lock().expect("poisoned").update_state(state),
            ItemValue::Sequence(sequence) => sequence.lock().expect("poisoned").update_state(state),
            ItemValue::Overlapped(overlapped) => {
                overlapped.lock().expect("poisoned").update_state(state)
            }
        }
    }

    pub(super) fn inner_duration(&self, state: &ToneGenerationState) -> Beat {
        match self {
            ItemValue::Note(note) => note.lock().expect("poisoned").inner_duration(state),
            ItemValue::Chord(chord) => chord.lock().expect("poisoned").inner_duration(state),
            ItemValue::Rest(rest) => rest.lock().expect("poisoned").duration(),
            ItemValue::Overlapped(overlapped) => {
                overlapped.lock().expect("poisoned").inner_duration(state)
            }
            ItemValue::Sequence(sequence) => sequence
                .lock()
                .expect("poisoned")
                .inner_duration(state.clone()),
            ItemValue::Scale(_) | ItemValue::Mode(_) | ItemValue::Set(_) => Beat::ZERO,
        }
    }
    pub fn length(&self) -> Beat {
        self.inner_length(&Default::default())
    }
    pub fn duration(&self) -> Beat {
        self.inner_duration(&Default::default())
    }

    pub fn parse(input: &str) -> IResult<&str, Self> {
        parse::item_value(input)
    }
}

impl FromStr for ItemValue {
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
    pub value: ItemValue,
}

impl Item {
    /// Resolve all the section's notes to playable instrument tones.
    /// The offset is the beat offset.
    pub(super) fn inner_tones<S>(
        &self,
        offset: Beat,
        metronome: &Metronome,
        pitch_standard: &S,
        state: &ToneGenerationState,
    ) -> Box<dyn Iterator<Item = Tone> + 'static>
    where
        S: PitchStandard + ?Sized,
    {
        self.value
            .inner_tones(offset, metronome, pitch_standard, state)
    }
    pub fn tones<S>(
        &self,
        offset: Beat,
        metronome: &Metronome,
        pitch_standard: &S,
    ) -> Box<dyn Iterator<Item = Tone> + 'static>
    where
        S: PitchStandard + ?Sized,
    {
        self.value.tones(offset, metronome, pitch_standard)
    }

    pub(super) fn inner_length(&self, state: &ToneGenerationState) -> Beat {
        self.value.inner_length(state)
    }

    pub(super) fn update_state(&self, state: &mut ToneGenerationState) {
        self.value.update_state(state)
    }

    pub(super) fn inner_duration(&self, state: &ToneGenerationState) -> Beat {
        self.value.inner_duration(state)
    }
    pub fn length(&self) -> Beat {
        self.inner_length(&Default::default())
    }
    pub fn duration(&self) -> Beat {
        self.inner_duration(&Default::default())
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
