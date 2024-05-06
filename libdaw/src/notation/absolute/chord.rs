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

/// An absolute chord, contextually relevant.
#[derive(Debug, Clone)]
pub struct Chord {
    pub pitches: Vec<Arc<Mutex<Pitch>>>,

    // Conceptual length of the chord in beats
    pub length: Option<Beat>,

    // Actual playtime of the chord in beats, which will default to the length
    // usually.
    pub duration: Option<Beat>,
}

impl Chord {
    /// Resolve all the section's chords to playable instrument tones.
    /// The offset is the beat offset.
    pub fn resolve<S>(
        &self,
        offset: Beat,
        metronome: &Metronome,
        pitch_standard: &S,
        previous_length: Beat,
    ) -> impl Iterator<Item = Tone> + 'static
    where
        S: PitchStandard + ?Sized,
    {
        let start = metronome.beat_to_time(offset);
        let duration = self.duration(previous_length);
        let end_beat = offset + duration;
        let end = metronome.beat_to_time(end_beat);
        let length = end - start;
        let pitches: Vec<_> = self
            .pitches
            .iter()
            .map(move |pitch| {
                let pitch = pitch.lock().expect("poisoned");
                let frequency = pitch_standard.resolve(&pitch);
                Tone {
                    start,
                    length,
                    frequency,
                }
            })
            .collect();
        pitches.into_iter()
    }

    pub fn length(&self, previous_length: Beat) -> Beat {
        self.length.unwrap_or(previous_length)
    }

    pub fn duration(&self, previous_length: Beat) -> Beat {
        self.duration.or(self.length).unwrap_or(previous_length)
    }

    pub fn parse(input: &str) -> IResult<&str, Self> {
        parse::chord(input)
    }

    pub fn deep_clone(&self) -> Self {
        Self {
            pitches: self
                .pitches
                .iter()
                .map(|pitch| Arc::new(Mutex::new(pitch.lock().expect("poisoned").deep_clone())))
                .collect(),
            length: self.length,
            duration: self.duration,
        }
    }
}

impl FromStr for Chord {
    type Err = Error<String>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let chord = all_consuming(parse::chord)(s)
            .finish()
            .map_err(|e| e.to_owned())?
            .1;
        Ok(chord)
    }
}
