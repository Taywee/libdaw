mod parse;

use super::{
    tone_generation_state::ToneGenerationState, Duration, Element, NotePitch, StateMember,
};
use crate::{
    metronome::{Beat, Metronome},
    nodes::instrument::Tone,
    parse::IResult,
    pitch::PitchStandard,
};
use nom::{combinator::all_consuming, error::convert_error, Finish as _};
use std::str::FromStr;

/// An absolute chord, contextually relevant.
#[derive(Debug, Clone)]
pub struct Chord {
    // An empty chord is just a rest.
    pub pitches: Vec<NotePitch>,

    // Conceptual length of the chord in beats
    pub length: Option<Beat>,

    // Actual playtime of the chord in beats, which will default to the length
    // usually.
    pub duration: Option<Duration>,

    pub state_member: Option<StateMember>,
}

impl Element for Chord {
    /// Resolve all the section's chords to playable instrument tones.
    /// The offset is the beat offset.
    fn tones(
        &self,
        metronome: &Metronome,
        pitch_standard: &dyn PitchStandard,
        state: &ToneGenerationState,
    ) -> Box<dyn Iterator<Item = Tone> + 'static> {
        let start = metronome.beat_to_time(state.offset);
        let duration = self.duration(state);
        let end_beat = state.offset + duration;
        let end = metronome.beat_to_time(end_beat);
        let length = end - start;
        let pitches: Vec<_> = self
            .pitches
            .iter()
            .map(move |pitch| {
                let frequency = pitch_standard.resolve(&pitch.absolute(state));
                Tone {
                    start,
                    length,
                    frequency,
                    tags: state.tags.clone(),
                }
            })
            .collect();
        Box::new(pitches.into_iter())
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
        match self.state_member {
            Some(StateMember::First) => self.pitches[0].update_state(state),
            Some(StateMember::Last) => {
                self.pitches.last().unwrap().update_state(state);
            }
            None => (),
        }
        if let Some(length) = self.length {
            state.length = length;
        }
        if let Some(duration) = self.duration {
            state.duration = duration;
        }
        state.offset += state.length;
    }
}
impl Chord {
    pub fn parse(input: &str) -> IResult<&str, Self> {
        parse::chord(input)
    }
}

impl FromStr for Chord {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let chord = all_consuming(Self::parse)(s)
            .finish()
            .map_err(move |e| convert_error(s, e))?
            .1;
        Ok(chord)
    }
}
