use crate::{
    metronome::{Beat, Metronome},
    nodes::instrument::Tone,
    parse::{notation::absolute as parse, Error},
    pitch::{Pitch, PitchStandard},
};
use nom::{combinator::all_consuming, Finish as _};
use std::str::FromStr;

/// An absolute note, contextually relevant.
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct Note {
    pub pitch: Pitch,

    // Conceptual length of the note in beats
    pub length: Beat,

    // Actual playtime of the note in beats, which will default to the length
    // usually.
    pub duration: Option<Beat>,
}

impl Note {
    /// Resolve all the section's notes to playable instrument tones.
    /// The offset is the beat offset.
    pub fn resolve<S>(&self, offset: Beat, metronome: &Metronome, standard: &S) -> Tone
    where
        S: PitchStandard + ?Sized,
    {
        let frequency = standard.resolve(self.pitch);
        let start = metronome.beat_to_time(offset);
        let duration = self.duration.unwrap_or(self.length);
        let end_beat = offset + duration;
        let end = metronome.beat_to_time(end_beat);
        let length = end - start;
        Tone {
            start,
            length,
            frequency,
        }
    }

    pub fn length(&self) -> Beat {
        self.length
    }

    pub fn duration(&self) -> Beat {
        self.duration.unwrap_or(self.length)
    }
}

impl FromStr for Note {
    type Err = Error<String>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let note = all_consuming(parse::note)(s)
            .finish()
            .map_err(|e| e.to_owned())?
            .1;
        Ok(note)
    }
}
