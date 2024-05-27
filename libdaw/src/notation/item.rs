mod parse;

use super::{
    tone_generation_state::ToneGenerationState, Chord, Inversion, Note, Overlapped, Rest, Scale,
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

#[derive(Clone)]
pub enum Item {
    Note(Arc<Mutex<Note>>),
    Chord(Arc<Mutex<Chord>>),
    Rest(Arc<Mutex<Rest>>),
    Overlapped(Arc<Mutex<Overlapped>>),
    Sequence(Arc<Mutex<Sequence>>),
    Scale(Arc<Mutex<Scale>>),
    Inversion(Arc<Mutex<Inversion>>),
    Set(Arc<Mutex<Set>>),
}

impl fmt::Debug for Item {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Item::Note(note) => fmt::Debug::fmt(&note.lock().expect("poisoned"), f),
            Item::Chord(chord) => fmt::Debug::fmt(&chord.lock().expect("poisoned"), f),
            Item::Rest(rest) => fmt::Debug::fmt(&rest.lock().expect("poisoned"), f),
            Item::Overlapped(overlapped) => {
                fmt::Debug::fmt(&overlapped.lock().expect("poisoned"), f)
            }
            Item::Sequence(sequence) => fmt::Debug::fmt(&sequence.lock().expect("poisoned"), f),
            Item::Scale(scale) => fmt::Debug::fmt(&scale.lock().expect("poisoned"), f),
            Item::Inversion(inversion) => fmt::Debug::fmt(&inversion.lock().expect("poisoned"), f),
            Item::Set(set) => fmt::Debug::fmt(&set.lock().expect("poisoned"), f),
        }
    }
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
        match self {
            Item::Note(note) => Box::new(std::iter::once(
                note.lock()
                    .expect("poisoned")
                    .inner_tone(offset, metronome, pitch_standard, state),
            )),
            Item::Chord(chord) => Box::new(chord.lock().expect("poisoned").inner_tones(
                offset,
                metronome,
                pitch_standard,
                state.clone(),
            )),
            Item::Overlapped(overlapped) => {
                Box::new(overlapped.lock().expect("poisoned").inner_tones(
                    offset,
                    metronome,
                    pitch_standard,
                    state.clone(),
                ))
            }
            Item::Sequence(sequence) => Box::new(sequence.lock().expect("poisoned").inner_tones(
                offset,
                metronome,
                pitch_standard,
                state.clone(),
            )),
            Item::Scale(_) | Item::Inversion(_) | Item::Rest(_) | Item::Set(_) => {
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
            Item::Note(note) => note.lock().expect("poisoned").inner_length(state),
            Item::Chord(chord) => chord.lock().expect("poisoned").inner_length(state),
            Item::Rest(rest) => rest.lock().expect("poisoned").inner_length(state),
            Item::Overlapped(overlapped) => {
                overlapped.lock().expect("poisoned").inner_length(state)
            }
            Item::Sequence(sequence) => sequence
                .lock()
                .expect("poisoned")
                .inner_length(state.clone()),
            Item::Scale(_) | Item::Inversion(_) | Item::Set(_) => Beat::ZERO,
        }
    }

    pub(super) fn update_state(&self, state: &mut ToneGenerationState) {
        match self {
            Item::Note(note) => note.lock().expect("poisoned").update_state(state),
            Item::Chord(chord) => chord.lock().expect("poisoned").update_state(state),
            Item::Rest(rest) => rest.lock().expect("poisoned").update_state(state),
            Item::Scale(scale) => scale.lock().expect("poisoned").update_state(state),
            Item::Inversion(inversion) => inversion.lock().expect("poisoned").update_state(state),
            Item::Set(set) => set.lock().expect("poisoned").update_state(state),
            Item::Sequence(sequence) => sequence.lock().expect("poisoned").update_state(state),
            Item::Overlapped(overlapped) => {
                overlapped.lock().expect("poisoned").update_state(state)
            }
        }
    }

    pub(super) fn inner_duration(&self, state: &ToneGenerationState) -> Beat {
        match self {
            Item::Note(note) => note.lock().expect("poisoned").inner_duration(state),
            Item::Chord(chord) => chord.lock().expect("poisoned").inner_duration(state),
            Item::Rest(rest) => rest.lock().expect("poisoned").duration(),
            Item::Overlapped(overlapped) => {
                overlapped.lock().expect("poisoned").inner_duration(state)
            }
            Item::Sequence(sequence) => sequence
                .lock()
                .expect("poisoned")
                .inner_duration(state.clone()),
            Item::Scale(_) | Item::Inversion(_) | Item::Set(_) => Beat::ZERO,
        }
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
