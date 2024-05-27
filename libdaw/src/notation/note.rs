mod parse;

use super::{tone_generation_state::ToneGenerationState, Duration, NotePitch};
use crate::{
    metronome::{Beat, Metronome},
    nodes::instrument::Tone,
    parse::IResult,
    pitch::PitchStandard,
};
use nom::{combinator::all_consuming, error::convert_error, Finish as _};
use std::str::FromStr;

/// An absolute note, contextually relevant.
#[derive(Debug, Clone)]
pub struct Note {
    pub pitch: NotePitch,

    // Conceptual length of the note in beats
    pub length: Option<Beat>,

    // Actual playtime of the note in beats, which will default to the length
    // usually.
    pub duration: Option<Duration>,
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
        let length = self.inner_length(state);
        let duration = self.duration.unwrap_or(state.duration);
        duration.resolve(length)
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
        self.pitch.update_state(state);
        if let Some(length) = self.length {
            state.length = length;
        }
        if let Some(duration) = self.duration {
            state.duration = duration;
        }
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
