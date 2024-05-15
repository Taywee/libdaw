mod parse;

use super::{Chord, Note, Overlapped, Rest, Sequence};
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
    pub fn resolve<S>(
        &self,
        offset: Beat,
        metronome: &Metronome,
        pitch_standard: &S,
        previous_length: Beat,
    ) -> Box<dyn Iterator<Item = Tone> + 'static>
    where
        S: PitchStandard + ?Sized,
    {
        match self {
            Item::Note(note) => Box::new(std::iter::once(note.lock().expect("poisoned").resolve(
                offset,
                metronome,
                pitch_standard,
                previous_length,
            ))),
            Item::Chord(chord) => Box::new(chord.lock().expect("poisoned").resolve(
                offset,
                metronome,
                pitch_standard,
                previous_length,
            )),
            Item::Rest(_) => Box::new(std::iter::empty()),
            Item::Overlapped(overlapped) => Box::new(overlapped.lock().expect("poisoned").resolve(
                offset,
                metronome,
                pitch_standard,
                previous_length,
            )),
            Item::Sequence(sequence) => Box::new(sequence.lock().expect("poisoned").resolve(
                offset,
                metronome,
                pitch_standard,
                previous_length,
            )),
        }
    }

    pub fn length(&self, previous_length: Beat) -> Beat {
        match self {
            Item::Note(note) => note.lock().expect("poisoned").length(previous_length),
            Item::Chord(chord) => chord.lock().expect("poisoned").length(previous_length),
            Item::Rest(rest) => rest.lock().expect("poisoned").length(previous_length),
            Item::Overlapped(overlapped) => {
                overlapped.lock().expect("poisoned").length(previous_length)
            }
            Item::Sequence(sequence) => sequence.lock().expect("poisoned").length(previous_length),
        }
    }

    pub fn next_previous_length(&self, previous_length: Beat) -> Beat {
        match self {
            Item::Note(note) => note.lock().expect("poisoned").length(previous_length),
            Item::Chord(chord) => chord.lock().expect("poisoned").length(previous_length),
            Item::Rest(rest) => rest.lock().expect("poisoned").length(previous_length),
            Item::Overlapped(_) => previous_length,
            Item::Sequence(_) => previous_length,
        }
    }

    pub fn duration(&self, previous_length: Beat) -> Beat {
        match self {
            Item::Note(note) => note.lock().expect("poisoned").duration(previous_length),
            Item::Chord(chord) => chord.lock().expect("poisoned").duration(previous_length),
            Item::Rest(rest) => rest.lock().expect("poisoned").duration(),
            Item::Overlapped(overlapped) => overlapped
                .lock()
                .expect("poisoned")
                .duration(previous_length),
            Item::Sequence(sequence) => {
                sequence.lock().expect("poisoned").duration(previous_length)
            }
        }
    }

    pub fn parse(input: &str) -> IResult<&str, Self> {
        parse::item(input)
    }

    pub fn deep_clone(&self) -> Self {
        match self {
            Item::Note(note) => Item::Note(Arc::new(Mutex::new(
                note.lock().expect("poisoned").deep_clone(),
            ))),
            Item::Chord(chord) => Item::Chord(Arc::new(Mutex::new(
                chord.lock().expect("poisoned").deep_clone(),
            ))),
            Item::Rest(rest) => {
                Item::Rest(Arc::new(Mutex::new(rest.lock().expect("poisoned").clone())))
            }
            Item::Overlapped(overlapped) => Item::Overlapped(Arc::new(Mutex::new(
                overlapped.lock().expect("poisoned").deep_clone(),
            ))),
            Item::Sequence(sequence) => Item::Sequence(Arc::new(Mutex::new(
                sequence.lock().expect("poisoned").deep_clone(),
            ))),
        }
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
