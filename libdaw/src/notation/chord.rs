mod parse;

use super::{tone_generation_state::ToneGenerationState, Duration, NotePitch, StateMember};
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

impl Chord {
    /// Resolve all the section's chords to playable instrument tones.
    /// The offset is the beat offset.
    pub(super) fn inner_tones<S>(
        &self,
        offset: Beat,
        metronome: &Metronome,
        pitch_standard: &S,
        mut state: ToneGenerationState,
    ) -> impl Iterator<Item = Tone> + 'static
    where
        S: PitchStandard + ?Sized,
    {
        let start = metronome.beat_to_time(offset);
        let duration = self.inner_duration(&state);
        let end_beat = offset + duration;
        let end = metronome.beat_to_time(end_beat);
        let length = end - start;
        let pitches: Vec<_> = self
            .pitches
            .iter()
            .map(move |pitch| {
                let frequency = pitch_standard.resolve(&pitch.absolute(&state));
                pitch.update_state(&mut state);
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
        self.inner_tones(offset, metronome, pitch_standard, Default::default())
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
        parse::chord(input)
    }
    pub(super) fn update_state(&self, state: &mut ToneGenerationState) {
        match self.state_member {
            Some(StateMember::First) => self.pitches[0].update_state(state),
            Some(StateMember::Last) => {
                for pitch in &self.pitches {
                    pitch.update_state(state);
                }
            }
            None => (),
        }
        if let Some(length) = self.length {
            state.length = length;
        }
        if let Some(duration) = self.duration {
            state.duration = duration;
        }
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
