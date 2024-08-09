mod parse;

use super::{tone_generation_state::ToneGenerationState, Duration, Element, NotePitch};
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

impl Element for Note {
    fn tones(
        &self,
        metronome: &Metronome,
        pitch_standard: &dyn PitchStandard,
        state: &ToneGenerationState,
    ) -> Box<dyn Iterator<Item = Tone> + 'static> {
        let frequency = pitch_standard.resolve(&self.pitch.absolute(state));
        let start = metronome.beat_to_time(state.offset);
        let duration = self.duration(state);
        let end_beat = state.offset + duration;
        let end = metronome.beat_to_time(end_beat);
        let length = end - start;
        Box::new(std::iter::once(Tone {
            start,
            length,
            frequency,
            tags: state.tags.clone(),
        }))
    }

    fn length(&self, state: &ToneGenerationState) -> Beat {
        self.length.unwrap_or(state.length)
    }

    fn duration(&self, state: &ToneGenerationState) -> Beat {
        let length = self.length(state);
        let duration = self.duration.unwrap_or(state.duration);
        duration.resolve(length)
    }
    fn update_state(&self, state: &mut ToneGenerationState) {
        self.pitch.update_state(state);
        if let Some(length) = self.length {
            state.length = length;
        }
        if let Some(duration) = self.duration {
            state.duration = duration;
        }
        state.offset += state.length;
    }
}

impl Note {
    pub fn parse(input: &str) -> IResult<&str, Self> {
        parse::note(input)
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
