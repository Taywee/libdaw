mod parse;

use super::{resolve_state::ResolveState, Chord, Note, Overlapped, Rest, Sequence};
use crate::{
    metronome::{Beat, Metronome},
    nodes::instrument::Tone,
    parse::{Error, IResult},
    pitch::PitchStandard,
};
use nom::{combinator::all_consuming, Finish as _};
use std::{
    str::FromStr,
    sync::{Arc, Mutex},
};

#[derive(Debug, Clone)]
pub enum Item {
    Note(Arc<Mutex<Note>>),
    Chord(Arc<Mutex<Chord>>),
    Rest(Arc<Mutex<Rest>>),
    Overlapped(Arc<Mutex<Overlapped>>),
    Sequence(Arc<Mutex<Sequence>>),
}

impl Item {
    /// Resolve all the section's notes to playable instrument tones.
    /// The offset is the beat offset.
    pub(super) fn inner_tones<S>(
        &self,
        offset: Beat,
        metronome: &Metronome,
        pitch_standard: &S,
        state: &ResolveState,
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
                state,
            )),
            Item::Rest(_) => Box::new(std::iter::empty()),
            Item::Overlapped(overlapped) => {
                Box::new(overlapped.lock().expect("poisoned").inner_tones(
                    offset,
                    metronome,
                    pitch_standard,
                    state,
                ))
            }
            Item::Sequence(sequence) => Box::new(sequence.lock().expect("poisoned").inner_tones(
                offset,
                metronome,
                pitch_standard,
                state.clone(),
            )),
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

    pub(super) fn inner_length(&self, state: &ResolveState) -> Beat {
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
        }
    }

    pub(super) fn update_state(&self, state: &mut ResolveState) {
        match self {
            Item::Note(note) => {
                let pitch = note.lock().expect("poisoned").absolute_pitch(state);
                let length = note.lock().expect("poisoned").inner_length(state);
                state.pitch = pitch;
                state.length = length;
            }
            Item::Chord(chord) => {
                state.length = chord.lock().expect("poisoned").inner_length(state)
            }
            Item::Rest(rest) => state.length = rest.lock().expect("poisoned").inner_length(state),
            Item::Overlapped(_) => (),
            Item::Sequence(_) => (),
        }
    }

    pub(super) fn inner_duration(&self, state: &ResolveState) -> Beat {
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
    type Err = Error<String>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let note = all_consuming(parse::item)(s)
            .finish()
            .map_err(|e| e.to_owned())?
            .1;
        Ok(note)
    }
}
