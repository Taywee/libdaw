mod parse;

use super::{tone_generation_state::ToneGenerationState, Pitch, Step};
use crate::{
    metronome::{Beat, Metronome},
    nodes::instrument::Tone,
    parse::IResult,
    pitch::{Pitch as AbsolutePitch, PitchStandard},
};
use nom::{combinator::all_consuming, error::convert_error, Finish as _};
use std::{
    fmt,
    str::FromStr,
    sync::{Arc, Mutex},
};

#[derive(Clone)]
pub enum NotePitch {
    Pitch(Arc<Mutex<Pitch>>),
    Step(Arc<Mutex<Step>>),
}

impl fmt::Debug for NotePitch {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            NotePitch::Pitch(pitch) => fmt::Debug::fmt(&pitch.lock().expect("poisoned"), f),
            NotePitch::Step(step) => fmt::Debug::fmt(&step.lock().expect("poisoned"), f),
        }
    }
}

impl NotePitch {
    pub fn parse(input: &str) -> IResult<&str, Self> {
        parse::note_pitch(input)
    }
    /// Resolve to an absolute pitch
    pub(super) fn absolute(&self, state: &ToneGenerationState) -> AbsolutePitch {
        match self {
            NotePitch::Pitch(pitch) => pitch.lock().expect("poisoned").absolute(state),
            NotePitch::Step(step) => step.lock().expect("poisoned").absolute(state),
        }
    }
    pub(super) fn update_state(&self, state: &mut ToneGenerationState) {
        let pitch = self.absolute(state);
        state.pitch = pitch;
        if let Self::Step(step) = self {
            step.lock().expect("poisoned").update_state(state);
        }
    }
}

impl FromStr for NotePitch {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let note = all_consuming(Self::parse)(s)
            .finish()
            .map_err(move |e| convert_error(s, e))?
            .1;
        Ok(note)
    }
}

/// An absolute note, contextually relevant.
#[derive(Debug, Clone)]
pub struct Note {
    pub pitch: NotePitch,

    // Conceptual length of the note in beats
    pub length: Option<Beat>,

    // Actual playtime of the note in beats, which will default to the length
    // usually.
    pub duration: Option<Beat>,
}

impl Note {
    /// Resolve all the section's notes to playable instrument tones.
    /// The offset is the beat offset.
    pub(super) fn inner_tone<S>(
        &self,
        offset: Beat,
        metronome: &Metronome,
        pitch_standard: &S,
        state: &ToneGenerationState,
    ) -> Tone
    where
        S: PitchStandard + ?Sized,
    {
        let frequency = pitch_standard.resolve(&self.pitch.absolute(state));
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

    pub(super) fn inner_length(&self, state: &ToneGenerationState) -> Beat {
        self.length.unwrap_or(state.length)
    }

    pub(super) fn inner_duration(&self, state: &ToneGenerationState) -> Beat {
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

    pub(super) fn update_state(&self, state: &mut ToneGenerationState) {
        let length = self.inner_length(state);
        self.pitch.update_state(state);
        state.length = length;
    }
}

impl FromStr for Note {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let note = all_consuming(Self::parse)(s)
            .finish()
            .map_err(move |e| convert_error(s, e))?
            .1;
        Ok(note)
    }
}
