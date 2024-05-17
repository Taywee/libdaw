mod parse;

use super::{resolve_state::ResolveState, Pitch};
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
    pub(super) fn inner_tones<S>(
        &self,
        offset: Beat,
        metronome: &Metronome,
        pitch_standard: &S,
        state: &ResolveState,
    ) -> impl Iterator<Item = Tone> + 'static
    where
        S: PitchStandard + ?Sized,
    {
        let mut state = state.clone();
        let start = metronome.beat_to_time(offset);
        let duration = self.inner_duration(&state);
        let end_beat = offset + duration;
        let end = metronome.beat_to_time(end_beat);
        let length = end - start;
        let pitches: Vec<_> = self
            .pitches
            .iter()
            .map(move |pitch| {
                let pitch = pitch.lock().expect("poisoned").absolute(&state);
                let frequency = pitch_standard.resolve(&pitch);
                state.pitch = pitch;
                Tone {
                    start,
                    length,
                    frequency,
                }
            })
            .collect();
        pitches.into_iter()
    }

    /// Resolve all the section's chords to playable instrument tones.
    /// The offset is the beat offset.
    pub fn tones<S>(
        &self,
        offset: Beat,
        metronome: &Metronome,
        pitch_standard: &S,
    ) -> impl Iterator<Item = Tone> + 'static
    where
        S: PitchStandard + ?Sized,
    {
        self.inner_tones(offset, metronome, pitch_standard, &Default::default())
    }

    pub(super) fn inner_length(&self, state: &ResolveState) -> Beat {
        self.length.unwrap_or(state.length)
    }

    pub(super) fn inner_duration(&self, state: &ResolveState) -> Beat {
        self.duration.or(self.length).unwrap_or(state.length)
    }
    pub fn length(&self) -> Beat {
        self.inner_length(&Default::default())
    }
    pub fn duration(&self) -> Beat {
        self.inner_duration(&Default::default())
    }

    pub fn parse(input: &str) -> IResult<&str, Self> {
        parse::chord(input)
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
