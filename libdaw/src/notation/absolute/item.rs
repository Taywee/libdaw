use super::{Chord, Note, Overlapped, Rest};
use crate::{
    metronome::{Beat, Metronome},
    nodes::instrument::Tone,
    parse::{notation::absolute as parse, Error},
    pitch::PitchStandard,
};
use nom::{combinator::all_consuming, Finish as _};
use std::str::FromStr;

#[derive(Debug, Clone, PartialEq)]
pub enum Item {
    Note(Note),
    Chord(Chord),
    Rest(Rest),
    Overlapped(Overlapped),
}

impl Item {
    /// Resolve all the section's notes to playable instrument tones.
    /// The offset is the beat offset.
    pub fn resolve<'a, S>(
        &'a self,
        offset: Beat,
        metronome: &'a Metronome,
        standard: &'a S,
        default_length: Beat,
    ) -> Box<dyn Iterator<Item = Tone> + 'a>
    where
        S: PitchStandard + ?Sized,
    {
        match self {
            Item::Note(note) => Box::new(std::iter::once(note.resolve(
                offset,
                metronome,
                standard,
                default_length,
            ))),
            Item::Chord(chord) => {
                Box::new(chord.resolve(offset, metronome, standard, default_length))
            }
            Item::Rest(_) => Box::new(std::iter::empty()),
            Item::Overlapped(overlapped) => {
                Box::new(overlapped.resolve(offset, metronome, standard))
            }
        }
    }

    pub fn length(&self, default: Beat) -> Beat {
        match self {
            Item::Note(note) => note.length(default),
            Item::Chord(chord) => chord.length(default),
            Item::Rest(rest) => rest.length(default),
            Item::Overlapped(overlapped) => overlapped.length(),
        }
    }

    pub fn duration(&self, default_length: Beat) -> Beat {
        match self {
            Item::Note(note) => note.duration(default_length),
            Item::Chord(chord) => chord.duration(default_length),
            Item::Rest(rest) => rest.duration(),
            Item::Overlapped(overlapped) => overlapped.duration(),
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
