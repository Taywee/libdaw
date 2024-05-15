mod parse;

use crate::{
    metronome::{Beat, Metronome},
    nodes::instrument::Tone,
    parse::{Error, IResult},
    pitch::{Pitch, PitchStandard},
};
use nom::{combinator::all_consuming, Finish as _};
use std::{
    str::FromStr,
    sync::{Arc, Mutex},
};

/// An absolute note, contextually relevant.
#[derive(Debug, Clone)]
pub struct Note {
    pub pitch: Arc<Mutex<Pitch>>,

    // Conceptual length of the note in beats
    pub length: Option<Beat>,

    // Actual playtime of the note in beats, which will default to the length
    // usually.
    pub duration: Option<Beat>,
}

impl Note {
    /// Resolve all the section's notes to playable instrument tones.
    /// The offset is the beat offset.
    pub fn resolve<S>(
        &self,
        offset: Beat,
        metronome: &Metronome,
        pitch_standard: &S,
        previous_length: Beat,
    ) -> Tone
    where
        S: PitchStandard + ?Sized,
    {
        let frequency = pitch_standard.resolve(&self.pitch.lock().expect("poisoned"));
        let start = metronome.beat_to_time(offset);
        let duration = self.duration(previous_length);
        let end_beat = offset + duration;
        let end = metronome.beat_to_time(end_beat);
        let length = end - start;
        Tone {
            start,
            length,
            frequency,
        }
    }

    pub fn length(&self, previous_length: Beat) -> Beat {
        self.length.unwrap_or(previous_length)
    }

    pub fn duration(&self, previous_length: Beat) -> Beat {
        self.duration.or(self.length).unwrap_or(previous_length)
    }

    pub fn parse(input: &str) -> IResult<&str, Self> {
        parse::note(input)
    }
    pub fn deep_clone(&self) -> Self {
        Self {
            pitch: Arc::new(Mutex::new(
                self.pitch.lock().expect("poisoned").deep_clone(),
            )),
            length: self.length,
            duration: self.duration,
        }
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
