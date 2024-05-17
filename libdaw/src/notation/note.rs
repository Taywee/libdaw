mod parse;

use super::{resolve_state::ResolveState, Pitch};
use crate::{
    metronome::{Beat, Metronome},
    nodes::instrument::Tone,
    parse::{Error, IResult},
    pitch::{Pitch as AbsolutePitch, PitchStandard},
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
    pub fn absolute_pitch(&self, state: &ResolveState) -> AbsolutePitch {
        self.pitch.lock().expect("poisoned").absolute(state)
    }

    /// Resolve all the section's notes to playable instrument tones.
    /// The offset is the beat offset.
    pub(super) fn inner_tone<S>(
        &self,
        offset: Beat,
        metronome: &Metronome,
        pitch_standard: &S,
        state: &ResolveState,
    ) -> Tone
    where
        S: PitchStandard + ?Sized,
    {
        let frequency = pitch_standard.resolve(&self.absolute_pitch(state));
        let start = metronome.beat_to_time(offset);
        let duration = self.inner_duration(state);
        let end_beat = offset + duration;
        let end = metronome.beat_to_time(end_beat);
        let length = end - start;
        Tone {
            start,
            length,
            frequency,
        }
    }

    /// Resolve all the section's notes to playable instrument tones.
    /// The offset is the beat offset.
    pub fn tone<S>(&self, offset: Beat, metronome: &Metronome, pitch_standard: &S) -> Tone
    where
        S: PitchStandard + ?Sized,
    {
        self.inner_tone(offset, metronome, pitch_standard, &Default::default())
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
        parse::note(input)
    }
}

impl FromStr for Note {
    type Err = Error<String>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let note = all_consuming(Self::parse)(s)
            .finish()
            .map_err(|e| e.to_owned())?
            .1;
        Ok(note)
    }
}
